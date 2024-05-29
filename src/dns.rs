use rustdns::Message;
use rustdns::types::*;
use std::io::Write;
use std::net::UdpSocket;
use std::time::Duration;

use crate::gandi::Gandi;
use crate::print_green;
use crate::print_red;
use crate::print_yellow;
use crate::simplify_domain;

use rand::prelude::SliceRandom;

enum SpfResult {
    SoftFail,
    Missing
}

struct SpfCacheRecord {
    domain: String,
    result: SpfResult
}

enum DmarcResult {
    None,
    Missing
}

struct DmarcCacheRecord {
    domain: String,
    result: DmarcResult
}

pub struct DnsChecker {
    dns_servers: Vec<String>,
    spf_record_cache: Vec<SpfCacheRecord>,
    dmarc_record_cache: Vec<DmarcCacheRecord>
}

impl DnsChecker {
    //create a new DnsChecker
    pub fn new(dns_servers: Vec<String>) -> DnsChecker {
        DnsChecker {
            dns_servers,
            spf_record_cache: Vec::new(),
            dmarc_record_cache: Vec::new()
        }
    }

    pub async fn spf_records_check(&mut self, domain: &str, gandi: &mut Gandi) -> Result<(), String> {
        // A DNS Message can be easily constructed
        let mut m = Message::default();
        m.add_question(domain, Type::TXT, Class::Internet);
        m.add_extension(Extension {   // Optionally add a EDNS extension
            payload_size: 4096,       // which supports a larger payload size.
            ..Default::default()
        });
    
        // Setup a UDP socket for sending to a DNS server.
        let socket = UdpSocket::bind("0.0.0.0:0")
            .map_err(|e| e.to_string())?;
        socket.set_read_timeout(Some(Duration::new(5, 0)))
            .map_err(|e| e.to_string())?;

        //pick a random dns server
        let dns_server = self.dns_servers.choose(&mut rand::thread_rng()).unwrap().clone();
        let dns_server = format!( "{}:53", dns_server);
        
        socket.connect(&dns_server)
            .map_err(|e| e.to_string())?;
    
        // Encode the DNS Message as a Vec<u8>.
        let question = m.to_vec()
            .map_err(|e| e.to_string())?;
    
        // Send to the server.
        socket.send(&question)
            .map_err(|e| e.to_string())?;
    
        // Wait for a response from the DNS server.
        let mut resp = [0; 4096];
        let len = socket.recv(&mut resp)
            .map_err(|e| e.to_string())?;
    
        // Take the response bytes and turn it into another DNS Message.
        let answer = Message::from_slice(&resp[0..len])
            .map_err(|e| e.to_string())?;
    
        let mut found = false;
    
        for record in answer.answers {
            let record_string = record.resource.to_string();
    
            //strip out " "
            let record_string = record_string.replace("\" \"", "");
    
            if !record_string.contains("v=spf") {
                continue
            }
    
            found = true;
    
            //does the record end in ~all
            if record_string.contains("~all") {
                print_yellow(&format!("  {} has a softfail spf record", domain));
                //cache
                self.spf_record_cache.push(SpfCacheRecord { domain: domain.to_string(), result: SpfResult::SoftFail });
            }
    
    
            //next extract all include statements using regex searching for "include:" and ending with a space, ", or end of line
            let re = regex::Regex::new(r"include:[^ ,]*").unwrap();
            for cap in re.captures_iter(&record_string) {
                let include = &cap[0].replace("include:", "");
                if include == "spf.protection.outlook.com" {
                    continue; //skip the outlook include, its secure and reduces noise
                }
                print!("Found spf include: {} ... ", include);
                std::io::stdout().flush().unwrap();
    
                //simplify the domain
                let include_old = include.clone();
                let include = simplify_domain(include).unwrap().to_string();
    
                //display a message if the domain was modified
                if include != include_old {
                    print!("converted to {} ... ", include);
                    std::io::stdout().flush().unwrap();
                }
    
                let status = gandi.check_domain(&include).await;
                let status = match status {
                    Ok(s) => { s },
                    Err(e) => {
                        return Err(e.to_string());
                    }
                };
    
                //print available in red
                if status == "available" {
                    print_red(&status);
                } else {
                    print_green(&status);
                }
            }
    
    
    
        }
    
        //if no spf record found
        if !found {
            print_yellow(&format!("  {} does not have an spf record", domain));
        }
    
    
        Ok(())
    }
    
    pub async fn dmarc_check(&mut self, domain: &str) -> Result <(), String> {
            // A DNS Message can be easily constructed
            let mut m = Message::default();
            m.add_question(&format!("_dmarc.{}",domain), Type::TXT, Class::Internet);
            m.add_extension(Extension {   // Optionally add a EDNS extension
                payload_size: 4096,       // which supports a larger payload size.
                ..Default::default()
            });
        
            // Setup a UDP socket for sending to a DNS server.
            let socket = UdpSocket::bind("0.0.0.0:0")
                .map_err(|e| e.to_string())?;
            socket.set_read_timeout(Some(Duration::new(5, 0)))
                .map_err(|e| e.to_string())?;

            //pick a random dns server
            let dns_server = self.dns_servers.choose(&mut rand::thread_rng()).unwrap().clone();
            let dns_server = format!( "{}:53", dns_server);
            
            socket.connect(&dns_server)
                .map_err(|e| e.to_string())?;
        
            // Encode the DNS Message as a Vec<u8>.
            let question = m.to_vec()
                .map_err(|e| e.to_string())?;
        
            // Send to the server.
            socket.send(&question)
                .map_err(|e| e.to_string())?;
        
            // Wait for a response from the DNS server.
            let mut resp = [0; 4096];
            let len = socket.recv(&mut resp)
                .map_err(|e| e.to_string())?;
        
            // Take the response bytes and turn it into another DNS Message.
            let answer = Message::from_slice(&resp[0..len])
                .map_err(|e| e.to_string())?;
            
            let mut found = false;
            for record in answer.answers {
                let record_string = record.resource.to_string();
                if record_string.contains("v=DMARC") && record_string.contains("p=none") {
                    print_yellow(&format!("  {} has a DMARC record with p=none", domain));
                }
    
                found = true;
            }   
    
            if !found {
                print_yellow(&format!("  {} does not have a DMARC record", domain));
            }
    
            Ok(())
    }    
}

