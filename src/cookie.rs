use url_parser::Url;

pub struct Cookie {
    pub value_: String,
    pub expires_: String,
    pub path_: String,
    pub domain_: String,
    pub secure_: String,
}

impl Cookie {
    pub fn clear (&self) {
        
    }
}