use url::Url;
use std::clone::Clone;

const DEFAULT_PORT: u16 = 80;
enum_from_primitive! {
#[derive(Debug, PartialEq)]
    pub enum Range {
        SCHEME = 0x01,
        NETLOC = 0x02,
        PATH = 0x04,
        PARAM = 0x08,
        QUERY = 0x10,
        FRAGMENT = 0x20,
        ALL = 0xFF,
        NONE = 0x00,
        SchemeNetlocPath = 0x07,
    }
}


#[derive(Debug, Clone, PartialOrd,Ord,PartialEq,Eq,Hash,)]
pub struct MyUrl {
    url_: Option<Url>,
}

impl MyUrl {
    pub fn new () -> MyUrl {
        MyUrl {
            url_ : None,
        }
    }

    pub fn parse (&mut self, url_str: &String) {
        self.url_ = match Url::parse (&url_str) {
            Ok(u) => Some(u),
            Err(_) => None,
        };
    }

    pub fn empty (&self) -> bool {
        if self.url_ == None {
            true
        }
        else {
            false
        }
    }

    pub fn get_net_loc (&self) -> String {
        let sf = self.clone();
        sf.url_.unwrap().host().unwrap().to_string()
    }

    pub fn get_url_str(&self, range: u8) -> String {
        //let range: u8 = range_ as u8;
        let mut url = String::new();
        if range & Range::SCHEME as u8 == Range::SCHEME as u8 && !self.url_.clone().unwrap().scheme().is_empty() {
            url = self.url_.clone().unwrap().scheme().to_string() + ":";
        }
        if range & Range::NETLOC as u8 == Range::NETLOC as u8 && !self.url_.clone().unwrap().host_str().unwrap().is_empty() {
            url = url + "//" + &self.url_.clone().unwrap().host_str().unwrap();
        }
        if range & Range::NETLOC as u8 == Range::NETLOC as u8 && self.url_.clone().unwrap().port().unwrap() != DEFAULT_PORT {
            url = url + ":" + &self.url_.clone().unwrap().port().unwrap().to_string();
        }
        if range & Range::PATH as u8 == Range::PATH as u8 {
            url = url + &self.url_.clone().unwrap().path();
        }
/*        if range & Range::PARAM as u8 == Range::PARAM as u8 && !self.url_.param().is_empty() {
            url = url + ";" + &self.url_.param();
        }
*/
        if range & Range::QUERY as u8 == Range::QUERY as u8 && self.url_.clone().unwrap().query() != None {
            url = url + "?" + &self.url_.clone().unwrap().query().unwrap();
        }
        if range & Range::FRAGMENT as u8 == Range::FRAGMENT as u8 && self.url_.clone().unwrap().fragment() != None {
            url = url + "#" + &self.url_.clone().unwrap().fragment().unwrap();
        }
        url
    }

    pub fn get_scheme (&self) -> String {
        self.url_.clone().unwrap().scheme().to_string()
    }

    pub fn update (&mut self, new_url: String) {
        self.url_ = match Url::parse (&new_url) {
            Ok(u) => Some(u),
            Err(_) => None,
        };
    }

    pub fn get_abs_path (&self, cur_url: &mut MyUrl) -> &MyUrl {
        *cur_url = self.clone();
        self
    }

    pub fn filter (&self) -> bool {
        if self.url_ == None {
            return false;
        }
        true
    }
    
    pub fn compare_netloc (&self, l_netloc : &String, r_netloc: &String) -> bool {
        let left = Url::parse(l_netloc).unwrap();
        let right = Url::parse(r_netloc).unwrap();

        if left.host() == right.host() && left.port() == right.port() {
            return true;
        }

        false
    }

    pub fn get_path (&self) -> String {
        self.url_.clone().unwrap().path().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chk_parse1() {
        let mut url = MyUrl::new();
        url.parse(&"http://www.jyoungsoft.com".to_string());
        assert_ne!(None, url.url_.clone());
        assert_eq!(url.url_.clone().unwrap().host_str().unwrap(), "www.jyoungsoft.com".to_string());
        println! ("MyUrl : {:?}", url.url_);
    }

    #[test]
    fn chk_parse2() {
        let mut url1 = MyUrl::new();
        let mut url2 = MyUrl::new();
        url1.parse(&"http://www.jyoungsoft.com:8080/aaa/bbb/1.html?param=aaa&param2=bbb".to_string());
        assert_ne!(None, url1.url_.clone());
        assert_eq!(url1.url_.clone().unwrap().host_str().unwrap(), "www.jyoungsoft.com".to_string());
        println! ("MyUrl : {:?}", url1.url_.clone());

        url2.parse(&"http://www.jyoungsoft.com:8080/aaa/bbb/ccc/2.html?param=aaa&param2=bbb".to_string());
        assert_ne!(None, url2.url_.clone());
        assert_eq!(url2.url_.clone().unwrap().host_str().unwrap(), "www.jyoungsoft.com".to_string());
        println! ("MyUrl : {:?}", url2.url_.clone());

        assert_ne!(url1, url2);
        assert_eq!(url1.url_.clone().unwrap().port(), url2.url_.clone().unwrap().port());
        assert_eq!(url1.url_.clone().unwrap().scheme(), url2.url_.clone().unwrap().scheme());
        assert_ne!(url1.url_.clone().unwrap().path(), url2.url_.clone().unwrap().path());
    }
}