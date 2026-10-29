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


fn main() {

    let url = String::from("http://exemple.com");
    let key = String::from("");

    match ShortLink::new(key, url) {
        Ok(shorty) => {
            println!(" key: {}",shorty.get_key());
            println!("url: {}", shorty.get_url())
        },
        Err(err) => eprintln!("{}",err)
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


