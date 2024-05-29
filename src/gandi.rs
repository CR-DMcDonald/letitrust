use core::panic;

use toml::Value;


pub struct Gandi {
    pat: String,
    domain_status_cache: Vec<(String, String)>
}

impl Gandi {
    pub fn new() -> Result<Gandi, String> {
        //get path to executable
        let path = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();
        std::env::set_current_dir(&path).unwrap();
        
        //read the config file
        let gandi: toml::Value = toml::from_str(&std::fs::read_to_string("config.toml").expect("Unable to read config.toml")).unwrap();
        let gandi_pat = gandi["gandi_pat"].as_str();

        let gandi_pat = match gandi_pat {
            Some(gp) => { gp },
            None => {
                return Err("gandi_pat not found in config.toml".to_string());
            }
        };

        //verify the key is 40 chracters of hexidecimal
        if gandi_pat.len() != 40 {
            return Err("gandi_pat is not 40 characters long".to_string());
        }
        if gandi_pat.chars().all(|c| c.is_ascii_hexdigit()) == false {
            return Err("gandi_pat is not hexidecimal".to_string());
        }
        Ok(Gandi { pat: gandi_pat.to_owned(), domain_status_cache: Vec::new()})
    }

    pub async fn check_domain(&mut self, domain: &str) -> Result<String, reqwest::Error> {
        //check if the domain is in the cache
        for (d, s) in &self.domain_status_cache {
            if d == domain {
                return Ok(s.to_string());
            }
        }

        let gandi = reqwest::Client::new();
        let res = gandi.get(&format!("https://api.gandi.net/v5/domain/check?name={}", domain))
            .header("Authorization", format!("Bearer {}", self.pat))
            .send().await;

        let res = match res {
            Ok(r) => { r },
            Err(e) => {
                panic!("Error: {}", e);
            }
        };

        let body = res.text().await.unwrap();

        // Parse the string of data into serde_json::Value.
        let v: Value = serde_json::from_str(&body).unwrap();

        // Navigate through the JSON to find the status.
        if let Some(products) = v["products"].as_array() {
            for product in products {
                if let Some(status) = product["status"].as_str() {
                    self.domain_status_cache.push((domain.to_string(), status.to_string()));
                    return Ok(status.to_string());
                }
            }
        }

        Ok("error".to_string())
    }
}