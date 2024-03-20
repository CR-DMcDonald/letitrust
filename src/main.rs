use std::io::Write;
use serde_json::Value;

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

    //create a vector for domain, status pairs
    let mut domain_status_cache: Vec<(String, String)> = Vec::new();

    if args[1] != "-f" && args[1] != "-d" {
        print_usage();
        return;
    }
    
    //get path to executable
    let path = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();
    std::env::set_current_dir(&path).unwrap();
    
    //read the config file
    let gandi: toml::Value = toml::from_str(&std::fs::read_to_string("config.toml").expect("Unable to read config.toml")).unwrap();
    let gandi_pat = gandi["gandi_pat"].as_str();

    let gandi_pat = match gandi_pat {
        Some(gp) => { gp },
        None => {
            print_red("gandi_pat not found in config.toml");
            return;
        }
    };

    //verify the key is 40 chracters of hexidecimal
    if gandi_pat.len() != 40 {
        print_red("gandi_pat is not 40 characters long");
        return;
    }
    if gandi_pat.chars().all(|c| c.is_ascii_hexdigit()) == false {
        print_red("gandi_pat is not hexidecimal");
        return;
    }

    //parse the key, check it looks good
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
        println!("Checking {}... ", domain);
        std::io::stdout().flush().unwrap();

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

            print!("Found domain: {} ... ", &cap[1]);
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

            //check if we have already checked this domain
            let mut found = false;
            for (d, status) in &domain_status_cache {
                if d == &domain {
                    if status == "available" {
                        print_red(status);
                    } else {
                        print_green(status);
                    }
                    found = true;
                    break;
                }
            }

            if found {
                continue;
            }

            //lookup gandi API, see if you can register the domain
            let gandi = reqwest::Client::new();
            let res = gandi.get(&format!("https://api.gandi.net/v5/domain/check?name={}", domain))
                .header("Authorization", format!("Bearer {}", gandi_pat))
                .send().await;
            
            let body = res.unwrap().text().await;

            if body.is_err() {
                println!(" error");
                continue;
            }

            let body = body.unwrap();

            // Parse the string of data into serde_json::Value.
            let v: Value = serde_json::from_str(&body).unwrap();

            // Navigate through the JSON to find the status.
            if let Some(products) = v["products"].as_array() {
                for product in products {
                    if let Some(status) = product["status"].as_str() {
                        if status == "available" {
                            domain_status_cache.push((domain.clone(), "available".to_string()));
                            print_red(status);
                        } else {
                            domain_status_cache.push((domain.clone(), status.to_string()));
                            print_green(status);
                        }
                    }
                }
            }
        }
            
    }

}

fn print_green(text: &str) {
    println!("\x1B[32m{}\x1B[0m", text);
}

fn print_red(text: &str) {
    println!("\x1B[31m{}\x1B[0m", text);
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

fn print_banner() {
    println!("");
    println!("██      ███████ ████████     ██ ████████     ██████  ██    ██ ███████ ████████ ");
    println!("██      ██         ██        ██    ██        ██   ██ ██    ██ ██         ██    ");
    println!("██      █████      ██        ██    ██        ██████  ██    ██ ███████    ██    ");
    println!("██      ██         ██        ██    ██        ██   ██ ██    ██      ██    ██    ");
    println!("███████ ███████    ██        ██    ██        ██   ██  ██████  ███████    ██    ");                                                                    
    println!("");                                                                  
    println!("letitrust v0.1");
    println!("Written by Darren McDonald, Cryptic Red");
}

fn print_usage() {
    println!("");
    println!("Usage: ./letitrust -f <filename>");
    println!("       ./letitrust -d <domain>");
}