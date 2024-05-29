mod dns;
mod gandi;
mod util;

use std::io::Write;
use gandi::Gandi;
use serde_json::Value;
use util::*;

#[tokio::main]
async fn main() {
    //print the banner
    print_banner();

    //get file name from command line args
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        print_usage();
        return;
    }

    if args[1] != "-f" && args[1] != "-d" {
        print_usage();
        return;
    }

    //parse the key, check it looks good
    let mut gandi = match Gandi::new() {
        Ok(g) => { g },
        Err(e) => {
            print_red(&e);
            return;
        }
    };

    let mut dns_checker = dns::DnsChecker::new(vec!["8.8.8.8".to_string()]);

    let domainlist: Vec<String>;
    if args[1] == "-d" {
        domainlist = args[2].clone().split(",").map(|s| s.to_string()).collect();
    }
    else {
        domainlist = std::fs::read_to_string(args[2].clone()).expect("Unable to read file").split("\n").map( |s| s.to_string() ).collect();
    }

    //iterate through the list of domains
    for domain in domainlist {
        println!("");
        //print the domain
        print!("Checking {}... ", domain);
        std::io::stdout().flush().unwrap();

        //lookup gandi API, see if you can register the domain
        let status = gandi.check_domain(&domain).await;
        let status = match status {
            Ok(s) => { s },
            Err(e) => {
                print_red(e.to_string().as_str());
                continue;
            }
        };

        if status == "available" {
            print_red(&status);
        } else {
            print_green(&status);
        }        

        //check spf records
        let spf_result = dns_checker.spf_records_check(&domain, &mut gandi).await;
        match spf_result {
            Ok(s) => { s },
            Err(e) => {
                print_red(&e);
                continue;
            }
        };

        //check dmarc records
        let dmarc_result = dns_checker.dmarc_check(&domain).await;
        match dmarc_result {
            Ok(()) => {},
            Err(e) => {
                print_red(&e);
                continue;
            }
        };

        let xml_string = format!(r#"<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:exm="http://schemas.microsoft.com/exchange/services/2006/messages" xmlns:ext="http://schemas.microsoft.com/exchange/services/2006/types" xmlns:a="http://www.w3.org/2005/08/addressing" xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
    <soap:Header>
        <a:Action soap:mustUnderstand="1">http://schemas.microsoft.com/exchange/2010/Autodiscover/Autodiscover/GetFederationInformation</a:Action>
        <a:To soap:mustUnderstand="1">https://autodiscover-s.outlook.com/autodiscover/autodiscover.svc</a:To>
        <a:ReplyTo>
            <a:Address>http://www.w3.org/2005/08/addressing/anonymous</a:Address>
        </a:ReplyTo>
    </soap:Header>
    <soap:Body>
        <GetFederationInformationRequestMessage xmlns="http://schemas.microsoft.com/exchange/2010/Autodiscover">
            <Request>
                <Domain>{}</Domain>
            </Request>
        </GetFederationInformationRequestMessage>
    </soap:Body>
</soap:Envelope>"#, domain);

        //use reqwest to send the request
        let client = reqwest::Client::new();
        let res = client.post("https://autodiscover-s.outlook.com/autodiscover/autodiscover.svc")
            .header("Content-Type", "text/xml; charset=utf-8")
            .header("SOAPAction", "http://schemas.microsoft.com/exchange/2010/Autodiscover/Autodiscover/GetFederationInformation")
            .header("User-Agent", "AutodiscoverClient")
            .body(xml_string)
            .send().await;

        let body = match res {
            Ok(r) => {
                r.text().await
            },
            Err(e) => {
                print_red(" error");
                continue;
            }
        };

        let body = match body {
            Ok(b) => {
                b
            },
            Err(e) => {
                print_red(" error");
                continue;
            }
        };
        
        //extract each domain from the response, which is <Domain> </Domain>
        let re = regex::Regex::new(r"<Domain>(.*?)</Domain>").unwrap();
        for cap in re.captures_iter(&body) {
            //extract domain
            let domain = &cap[1].to_string();
            let domain_clone = domain.clone();
            let domain = simplify_domain(domain).unwrap().to_string();

            print!("Found Azure Tenant domain: {} ... ", &cap[1]);
            std::io::stdout().flush().unwrap();

            //skip onmicrosoft.com
            if domain.contains("onmicrosoft.com") {
                println!("skipped");
                continue;
            }

            //display a message if the domain was modified
            if domain != domain_clone {
                print!("converted to {} ... ", domain);
                std::io::stdout().flush().unwrap();
            }

            //lookup gandi API, see if you can register the domain
            let status = gandi.check_domain(&domain).await;

            let status = match status {
                Ok(s) => { s },
                Err(e) => {
                    print_red(e.to_string().as_str());
                    continue;
                }
            };

            //print the status
            if status == "available" {
                print_red(&status);
            } else {
                print_green(&status);
            }

            //check spf records
            let spf_result = dns_checker.spf_records_check(&domain, &mut gandi).await;
            match spf_result {
                Ok(s) => { s },
                Err(e) => {
                    print_red(&e);
                    continue;
                }
            };

            //check dmarc records
            let dmarc_result = dns_checker.dmarc_check(&domain).await;
            match dmarc_result {
                Ok(()) => {},
                Err(e) => {
                    print_red(&e);
                    continue;
                }
            };

        }
            
    }

}


fn simplify_domain(domain: &str) -> Option<String> {
    let parts: Vec<&str> = domain.split('.').collect();
    // Check if the domain has enough parts to potentially include a subdomain and a TLD/SLD
    if parts.len() > 2 {
        // Identify common second-level domains (SLDs)
        let slds = ["co", "com", "net", "org", "gov", "nhs", "edu", "ac", "biz"];
        let second_to_last = parts[parts.len() - 2];
        if slds.contains(&second_to_last) {
            // If the second-to-last part is a common SLD, assume the last two parts are the domain and TLD
            // and the third-to-last part is the main domain name.
            return Some(format!("{}.{}.{}", parts[parts.len() - 3], parts[parts.len() - 2], parts[parts.len() - 1]));
        } else {
            // If not a recognized SLD, assume the last part is the TLD and the second-to-last part is the domain
            return Some(format!("{}.{}", parts[parts.len() - 2], parts[parts.len() - 1]));
        }
    } else if parts.len() == 2 {
        // If there are exactly two parts, it's already simplified
        return Some(domain.to_string());
    }
    // If it's a single part or otherwise doesn't match the conditions, return None
    None
}
