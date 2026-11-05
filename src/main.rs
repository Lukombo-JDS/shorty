#![allow(dead_code, unused_variables, unused_imports)]

use std::{collections::HashMap, fs::{self, File}, io::{BufRead, BufReader, Read}, path::PathBuf};
use rand::distr::{Alphanumeric, SampleString};
use url::Url;
use std::fmt::Display;
use serde::{Deserialize, Serialize, Serializer};

const NB_KEY_CHARACTERS: usize = 12;

/// Object saving user url and its key.
#[derive(Serialize,Deserialize)]
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

        if key.contains("http://") || key.contains("https://"){
            return Err(String::from("Error: key already have url scheme"))
        }
        
        match Url::parse(&url) {
            Ok(url_parsed) =>{
                
                if url_parsed.scheme() != "http" && url_parsed.scheme() != "https" {
                   return Err(String::from("Error: unsupported scheme"));
                }
        
                if url_parsed.host().is_none() {
                   return Err(String::from("Error: missing host"));
                }
                if url_parsed.scheme() == "http"{
                    let mut url_key = "http://".to_string();
                    url_key.push_str(&key);
                    return Ok(ShortLink{
                            url: url_parsed.to_string(),
                            key: url_key
                    })
                }else if url_parsed.scheme() == "https" {
                    let mut url_key = "https://".to_string();
                    url_key.push_str(&key);
                    return Ok(ShortLink{
                            url: url_parsed.to_string(),
                            key: url_key
                    })
                }else{
                    Err(String::from("Error: invalid url scheme"))
                }
                
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

#[derive(Serialize,Deserialize,Debug)]
struct ShortStore{
    links: HashMap<String, String>
}

// #[derive(Serialize)]
// struct InMemory{
//     store: ShortStore,
//     link: ShortLink
//     
// }
// 
// impl InMemory{
//     fn new(store: ShortStore, link: ShortLink)->InMemory{
//         InMemory { 
//             store: store,
//             link: link
//         }
//     }
// 
//     fn save(&self, path: &str)->Result<(),String>{
//         //serialize into json
// 
// 
//        match serde_json::to_string_pretty(self) {
//            Ok(key_url_json) => {
//                fs::write(path, key_url_json).or(Err(String::from("Error: can not write into file")))
//            },
//            Err(err)  =>  Err(String::from("Error: can not save the ShortLink"))
//        }
//     }
// }
// 


impl ShortStore{

    ///ShortStore Constructor for a new instance
    fn new()->ShortStore{
        ShortStore {
            links: HashMap::new(),
        }
    }

    ///ShortStore insert into HashMap (InMemory)
    fn insert(&mut self, link: ShortLink)->Result<(),String>{

        if self.links.contains_key(&link.key) {
           return Err(String::from("Error: key already exists")) // url exist for the specific key
        }

        self.links.insert(link.key, link.url);

        Ok(())
    }

    ///ShortStore generate an Alphanumeric key
    fn generate_key() -> String {
        Alphanumeric.sample_string(&mut rand::rng(), NB_KEY_CHARACTERS)
    }

    ///ShortStore shorten an url
    /// Return the shorten url as a key
    fn shorten(&mut self, url: String) -> Result<String, String> {
        self.shorten_with_generator(url, Self::generate_key)
    }

    ///ShortStore shorten logic 
    fn shorten_with_generator(&mut self, url: String, mut generate: impl FnMut() -> String)->Result<String,String>{

        for _ in 0..10 {
            
            let key_generated = generate();
            
            if !key_generated.contains("http://") && !key_generated.contains("https://"){
                
                let mut key_http = "http://".to_string();
                key_http.push_str(&key_generated);
                
                let mut key_https = "https://".to_string();
                key_https.push_str(&key_generated);

                if !self.links.contains_key(&key_http) && !self.links.contains_key(&key_https){
                    println!("key: {}",key_https);
                    let link = ShortLink::new(
                        key_generated.clone(),
                        url
                    )?;
                    let key = link.key.clone();
                    println!("ShortLink key: {}",link.key);
                    println!("ShortLink url: {}", link.url);
                    self.insert(link)?;
                    
                    return Ok(key);
                }
                
            }
        }
        Err(String::from("Error: no free key found"))
    }

    ///ShortStore find the url with the given key
    fn resolve(&self, key: &str)->Option<&str>{
        self.links.get(key).map(String::as_str)
    }

    ///ShortStore save link into a file
    fn save(&self, path: &str) -> Result<(),String>{
        self.save_with_writer(path, Self::writer)
        
    }

    //Writer for save function
    fn writer(path:&str, contents:&str) -> Result<(), String> {
      fs::write(path, contents).or(Err("can not write store into file".to_string()))
    }

    ///Save store logic
    ///save the store into a file.
    fn save_with_writer(&self, path:&str, writer: impl Fn(&str,&str) -> Result<(),String>) -> Result<(),String> {

        serde_json::to_string_pretty(self).map_or_else(
            |err| {
                Err(err.to_string())
            },
            |content| {
                writer(path,&content)
            }
        )
    }

    fn deserializer(content: &str)->Self{
        let store:Self = match serde_json::from_str(content){
            Ok(c) => c,
            Err(_) => ShortStore{
                links: HashMap::new(),
            }
        };
        store
    }

    fn load_with_deserializer(&self, path:&str, deserializer: impl Fn(&str)->ShortStore) -> Result<Self,String>{

        let f = fs::read_to_string(path).map_or_else(
            |err|{
                Err(err.to_string())
            },
           |r|{
               Ok(deserializer(&r))
           });
        f
    }

    /// ShortStore load from json store file: \
    /// Ok --> ShortStore \
    /// Err --> File's error \
    /// if error from desrialization --> ShortStore empty
    fn load(&self, path: &str) -> Result<Self, String> {

        self.load_with_deserializer(path, Self::deserializer)

//         let f = fs::read_to_string(path).map_or_else(
//             |err| {
//                 Err(err.to_string())
//             }, |r| {
//                let res:ShortStore = match serde_json::from_str(&r){
//                     Ok(resolve) => resolve,
//                     Err(_) => {
//                         // let mut error = "invalid desrialization: ".to_string();
//                         // error.push_str(&err.to_string());
//                         ShortStore { links: HashMap::new() }
//                     },
//                 };
// 
//                Ok(res)
//                
//             });
//         f
    }
}

fn main() {
    
    let pathbuf = PathBuf::from("shorty-links.json");
    println!("{}",pathbuf.display());

    let store = ShortStore::new();
    // store.shorten("https://discord.com".to_string()).expect("can not shorten the url");
    // store.shorten("https://youtube.com".to_string()).expect("can not shorten the url");
    let path = pathbuf.as_path().to_str().expect("path not found");
    // store.save(path).expect("can not save the store");
    let links_resolved = store.load(path).expect("Error: can not load the file");
    
    println!("{:?}",links_resolved)
    
}

///ShortStore's unit tests
#[cfg(test)]
mod tests_short_store {
    use std::path::PathBuf;
    use crate::{NB_KEY_CHARACTERS, ShortLink};
    use super::ShortStore;

    const DATA:&str = r#"
            {
                links: {
                    "abc123":"https://exemple.com,
                    "def456":"http://exemple_2.com"
                }
            }
        "#;

    /*
     * load Tests
     */


    
    #[test]
    fn verify_links_loaded_into_store(){

        let pathbuf = PathBuf::from("shorty-links.json");
        println!("{}",pathbuf.display());
    
        let mut store = ShortStore::new();
        store.shorten("https://discord.com".to_string()).expect("can not shorten the url");
        store.shorten("https://youtube.com".to_string()).expect("can not shorten the url");
        let path = pathbuf.as_path().to_str().expect("path not found");
        store.save(path).expect("can not save the store");
        let links_resolved = store.load(path).expect("Error: can not load the file");

        assert!(store.load(path).is_ok());
        assert_eq!(links_resolved.links.len(),2)
    }

    #[test]
    fn verify_links_loaded() {

        let pathbuf = PathBuf::from("shorty-links.json");
        println!("{}",pathbuf.display());
    
        let mut store = ShortStore::new();
        store.shorten("https://discord.com".to_string()).expect("can not shorten the url");
        store.shorten("https://youtube.com".to_string()).expect("can not shorten the url");
        let path = pathbuf.as_path().to_str().expect("path not found");
        store.save(path).expect("can not save the store");
        // let links_resolved = store.load(path).expect("Error: can not load the file");

        assert!(store.load(path).is_ok())
    }

    /*
     * save Tests
     */

    #[test]
    fn verify_store_consistency(){
        let mut store = ShortStore::new();
        
        let url_key = store.shorten("http://localhost".to_string())
            .expect("valid url: ");

        let writer = |p:&str, c:&str|{
                Ok(())
        };
        
        assert!(store.save_with_writer("store.json",writer).is_ok());
        assert_eq!(store.resolve(&url_key),Some("http://localhost/"))
        
    }
    
    #[test]
    fn verify_links_serialized(){

        let mut store = ShortStore::new();

        let _shorten = store.shorten("https://discord.com".to_string()).expect("Error: invalid url or no free key");

        let writer = |p:&str,c:&str|{
                Ok(())
            };

         assert_eq!(store.links.len(),1);
         assert!(store.save_with_writer("store.json",writer).is_ok())
        
    }
    
    /*
     * Shorten Tests
     */

    fn always_taken()->String{
        "Taken001".to_string()
    }

    #[test]
    fn rejects_after_10_attempts(){

        let mut short_store = ShortStore::new();

        let shorty = ShortLink::new(
            "Taken001".to_string(),
            "https://twitch.com".to_string()).expect("Error: can not create new link");

        println!("ShortLink key: {}", shorty.get_key());
        println!("ShortLink url: {}", shorty.get_url());

        short_store.insert(shorty).expect("Error: can not insert a new link");

        let mut calls = 0;
        let generate = || {
            calls+=1;
            always_taken()
        };

        let result = short_store.shorten_with_generator("https://go.com".to_string(), generate);

        assert_eq!(calls,10);
        assert!(result.is_err());
        assert_eq!(short_store.resolve("https://Taken001"), Some("https://twitch.com/"));
        assert_eq!(short_store.links.len(),1)
        
    }

    #[test]
    fn retries_after_collision () {

        let mut short_store = ShortStore::new();

        let shorty = ShortLink::new(
            "Taken001".to_string(),
            "https://twitch.com".to_string())
            .expect("Error: can not create new link");

        let _ = short_store.insert(shorty).expect("Error: can not insert link");

        let mut calls = 0;

        let generate = || {
            calls+=1;
            if calls == 1{
                "Taken001".to_string()
            }else{
                "Free0001".to_string()
            }
        };

        let result = short_store.shorten_with_generator(
            "https://exemple.com".to_string(), generate)
        .expect("Error: new try must found a free key");

         assert_eq!(calls,2);
         assert_eq!(result,"https://Free0001");
         assert_eq!(short_store.links.len(),2);
         assert_eq!(short_store.resolve("https://Free0001"),Some("https://exemple.com/"));
         assert_eq!(
             short_store.resolve("https://Taken001"),
             Some("https://twitch.com/")
         )
        
    }
    
    #[test]
    fn rejects_same_generate_key(){
        let mut short_store = ShortStore::new();

        let _r = short_store.shorten_with_generator(
            "https://exemple.com".to_string(), 
            always_taken
        ).expect("Error: can not generate a new key");

        let res = short_store.shorten_with_generator(
            "https://exemple_2.com".to_string(), 
            always_taken);

       assert!(res.is_err());
       assert_eq!(
           short_store.resolve("https://Taken001"),
           Some("https://exemple.com/")
       );
       assert_eq!(short_store.links.len(),1)
       
    }

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
        
        assert_eq!(key.len(), NB_KEY_CHARACTERS);
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
            shorty_store.links.get("https://abc123").map(String::as_str),
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
            shorty_store.links.get("https://abc123")
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
            shorty_store.resolve("https://ghi789"),
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
        
        assert_eq!(shorty.get_key(),"http://abc123");
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
    fn rejects_url_scheme_key_entry(){

        let r = ShortLink::new(
            "https://abc123".to_string(),
            "http://exemple.com".to_string());

        assert!(r.is_err())
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


