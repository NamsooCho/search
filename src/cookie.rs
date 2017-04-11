use url_parser::Url;

pub struct Cookie {
    pub value_: String,
    pub expires_: String,
    pub path_: String,
    pub domain_: String,
    pub secure_: String,
}

impl Cookie {
    pub fn new() -> Cookie {
        let mut c = Cookie {
            value_: String::new(),
            expires_: String::new(),
            path_: String::new(),
            domain_: String::new(),
            secure_: String::new(),
        };
        c
    }
    pub fn clear (&self) {
        
    }
}