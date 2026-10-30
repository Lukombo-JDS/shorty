use std::{collections::HashMap};

use rand::distr::{Alphanumeric, SampleString};
use url::Url;

/// Object saving user url and its key.
struct ShortLink{
    url: String,
    key: String
}

impl ShortLink {

    /// ShortLink Constructor: creating ShortLink instance
    /// with a url and a key
    fn new(key: String, url: String)->Result<Self,String>{

        
        if key.trim().is_empty() {
            return Err(String::from("Error: key empty"));
        }
        
        match Url::parse(&url) {
            Ok(url_parsed) =>{
                
                if url_parsed.scheme() != "http" && url_parsed.scheme() != "https" {
                   return Err(String::from("Error: unsupported scheme"));
                }
        
                if url_parsed.host().is_none() {
                   return Err(String::from("Error: missing host"));
                }
                
                Ok(ShortLink{
                        url: url_parsed.to_string(),
                        key: key 
                })
            },
            Err(err) => Err(err.to_string())
        }
    }

    ///Getting the stored url
    fn get_url(&self)->&str {
        &self.url
    }

    ///Getting the stored key
    fn get_key(&self)->&str{
        &self.key
    }
}

struct ShortStore{
    links: HashMap<String, String>,
}

impl ShortStore{

    ///ShortStore Constructor for a new instance
    fn new()->ShortStore{
        ShortStore { 
            links: HashMap::new(),
        }
    }

    fn insert(&mut self, link: ShortLink)->Result<(),String>{

        if self.links.contains_key(&link.key) {
           return Err(String::from("Error: key already exists")) // url exist for the specific key
        }

        self.links.insert(link.key, link.url);

        Ok(())
    }

    fn generate_key() -> String {
        Alphanumeric.sample_string(&mut rand::rng(), 8)
    }

    fn shorten(&mut self, url: String)->Result<String,String>{

        for _ in 0..10 {
            
            let key_generated = Self::generate_key();
            
            if !self.links.contains_key(&key_generated){
                
                let link = ShortLink::new(
                    key_generated.clone(),
                    url
                )?;
                
                self.insert(link)?;
                
                return Ok(key_generated);
            } 
        }
        Err(String::from("Error: no free key found"))
    }



    fn resolve(&self, key: &str)->Option<&str>{
        self.links.get(key).map(String::as_str)
    }
}

fn main() {

    
    
    
}

///ShortStore's unit tests
#[cfg(test)]
mod tests_short_store {
    use crate::ShortLink;
    use super::ShortStore;


    /*
     * Shorten Tests
     */

    #[test]
    fn shorten_stores_destination(){

        let mut short_store = ShortStore::new();

        let result = short_store.shorten("https://exemple.com/article".to_string());

        let key = result.expect("Error: can not shorten this url");

        let url_resolved = short_store.resolve(&key).expect("Error: url not found");

        assert_eq!(url_resolved,"https://exemple.com/article")
    }

    #[test]
    fn shorten_rejects_invalid_url(){

        let mut short_store = ShortStore::new();

        assert!(short_store.shorten("https://".to_string()).is_err());
        assert!(short_store.links.is_empty())
    }

    /*
     * Generate Key Tests
     */

    #[test]
    fn verify_generate_key_is_alphanumeric(){
        let key = ShortStore::generate_key();
        
        assert_eq!(key.len(), 8);
        assert!(key.chars().all(|character| character.is_ascii_alphanumeric()));
    }
    
    /*
     * Insert Tests
     */

    #[test]
    fn store_key_url() {
        let mut shorty_store = ShortStore::new();
        let shorty = ShortLink::new(
            String::from("abc123"),
            String::from("https://youtube.com")).expect("error: can not create link");
        
        let inserted = shorty_store.insert(shorty);
        assert!(inserted.is_ok());

        assert_eq!(
            shorty_store.links.get("abc123").map(String::as_str),
            Some("https://youtube.com/")
        )
        
    }

    #[test]
    fn rejects_same_key_insertion(){
        let mut shorty_store = ShortStore::new();
        let shorty_1 = ShortLink::new(
            String::from("abc123"),
            String::from("https://discord.com")).expect("error: can not create link");

        let _ = shorty_store
            .insert(shorty_1)
            .expect("the first insert must succeed");

        let shorty_2 = ShortLink::new(
            String::from("abc123"),
            String::from("https://youtube.com")).expect("error: can not create link");

        let inserted = shorty_store.insert(shorty_2);
        assert!(inserted.is_err());

        assert_eq!(
            shorty_store.links.get("abc123")
                .map(String::as_str),
            Some("https://discord.com/")
        )
        
    }

    #[test]
    fn return_the_right_url(){

        let shorty_1 = ShortLink::new(
            String::from("abc123"),
            String::from("https://youtube.com"));

        let shorty_2 = ShortLink::new(
            String::from("def456"), 
            String::from("http://localhost"));

        let shorty_3 = ShortLink::new(
            String::from("ghi789"),
            String::from("https://twitch.com"));

        let mut shorty_store = ShortStore::new();

        shorty_store.insert(shorty_1.expect("Error: invalid link"))
        .expect("invalid insert");

        shorty_store.insert(shorty_2.expect("Error: invalid link"))
            .expect("invalid insert");

        shorty_store.insert(shorty_3.expect("Error: invalid link"))
            .expect("invalid insert");

        assert_eq!(
            shorty_store.resolve("ghi789"),
            Some("https://twitch.com/")
        );

        assert_eq!(
            shorty_store.resolve("1234"),
            None
        )
        
        

        
    }
}

/// ShortLink's unit tests
#[cfg(test)]
mod tests {
    use super::ShortLink;

    #[test]
    fn rejects_empty_key() {
        let result = ShortLink::new(
            String::from(""),
            String::from("https://example.com"),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_whitespace_only_key() {
        let result = ShortLink::new(
            String::from("  "),
            String::from("http://exemple.com")
        );

        assert!(result.is_err())
    }

    #[test]
    fn rejects_blank_url(){
        let result = ShortLink::new(
            String::from("abc123"),
            String::from("  ")
        );

        assert!(result.is_err())
    }

    #[test]
    fn rejects_unsupported_url_sheme() {
        let result = ShortLink::new(
            String::from("abc123"), 
            String::from("ftp://exemple.com")
        );

        assert!(result.is_err())
    }

    #[test]
    fn rejects_missing_only_host() {
        let result = ShortLink::new(
            String::from("abc123"), 
            String::from("http://"));

        assert!(result.is_err())
    }
    

    #[test]
    fn store_valid_key_and_url() {
        
        let result = ShortLink::new(
            String::from("abc123"),
            String::from("http://exemple.com")
        );

        let shorty = result.expect("a valid key or url must be accepted");
        
        assert_eq!(shorty.get_key(),"abc123");
        assert_eq!(shorty.get_url(), "http://exemple.com/")
    }

    #[test]
    fn accepts_https_url(){

        let result = ShortLink::new(
            String::from("abc123"), 
            String::from("https://exemple.com/")
        );

        let shorty = result.expect("a valid key or url must be accepted");

        assert_eq!(shorty.get_url(), "https://exemple.com/")
    }

    #[test]
    fn normalize_url_without_path(){

        let result = ShortLink::new(
            String::from("abc123"), 
            String::from("https://exemple.com"),
        );

        let shorty = result.expect("a valid key or url must be accepted");

        assert_eq!(shorty.get_url(),"https://exemple.com/")
        
    }
}


