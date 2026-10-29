struct ShortLink{
    url: String,
    key: String
}

impl ShortLink {
    fn new(key: String, url: String)->Result<Self,String>{
       
        if key.trim().is_empty() {
            return Err(String::from("Error: key empty"));
        }

        if url.trim().is_empty() {
            return Err(String::from("Error: url empty"));
        }

        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(String::from("Error: invalid url scheme"));
        }

        Ok(ShortLink{ 
                url: url,
                key: key 
        })
    }

    fn get_url(&self)->&str {
        &self.url
    }

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
    fn preserves_valid_key_and_url() {
        
        let result = ShortLink::new(
            String::from("abc123"),
            String::from("http://exemple.com")
        );

        let shorty = result.expect("une clef valide doit être acceptée");
        
        assert_eq!(shorty.get_key(),"abc123");
        assert_eq!(shorty.get_url(), "http://exemple.com")
    }

}


