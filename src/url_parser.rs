use std::mem;
use std::ops::BitAnd;

const DEFAULT_PORT: u16 = 80;

enum Range {
    SCHEME = 0x01,
    NETLOC = 0x02,
    PATH = 0x04,
    PARAM = 0x08,
    QUERY = 0x10,
    FRAGMENT = 0x20,
    ALL = 0xFF,
}

pub struct Url {
    scheme_: String,
    pub net_loc_: String,
    path_: String,
    param_: String,
    query_: String,
    frag_: String,
    pub port_: u16,
}

impl Url {
    pub fn new () -> Url {
        let url = Url{scheme_: "".to_string(),
            net_loc_: "".to_string(),
            path_: "".to_string(),
            param_: "".to_string(),
            query_: "".to_string(),
            frag_: "".to_string(),
            port_: 80
            };
        url
    }

    fn get_url (&self, range_: Range) -> String {
        let range: u8 = range_ as u8;
        let mut url = String::new();
        if range & Range::SCHEME as u8 == Range::SCHEME as u8 && !self.scheme_.is_empty() {
            url = self.scheme_ + ":";
        }
        if range & Range::NETLOC as u8 == Range::NETLOC as u8 && !self.net_loc_.is_empty() {
            url = url + "//" + &self.net_loc_;
        }
        if range & Range::NETLOC as u8 == Range::NETLOC as u8 && self.port_ != DEFAULT_PORT {
            url = url + ":" + &self.port_.to_string();
        }
        if range & Range::PATH as u8 == Range::PATH as u8 {
            url = url + &self.path_;
            let path_tmp = self.path_.clone();
            if self.path_.len() > 1 && path_tmp.pop().unwrap() == '/' && self.path_.find('.') != None {
                url.truncate (url.rfind ('/').unwrap());
            }
        }
        if range & Range::PARAM as u8 == Range::PARAM as u8 && !self.param_.is_empty() {
            url = url + ";" + &self.param_;
        }
        if range & Range::QUERY as u8 == Range::QUERY as u8 && !self.query_.is_empty() {
            url = url + "?" + &self.query_;
        }
        if range & Range::FRAGMENT as u8 == Range::FRAGMENT as u8 && !self.frag_.is_empty() {
            url = url + "#" + &self.frag_;
        }
        url
    }

    pub fn empty (&self) -> bool {
        let range: Range = Range::ALL;
        let url_str = self.get_url(range);
        url_str.is_empty()
    }

    fn swap<'a> (&'a self, other: &'a Url) {
        mem::swap (&mut self, &mut other);
    }

    fn get_element (&self, url: &mut String, element: &mut String, c: char) {
        let pos = match url.find(c) {
            Some(p) => p,
            None => 0,
        };

        if 0 != pos {
            *element = url[pos+1..].to_string();
            url.truncate(pos);
        }
    }

    fn parse (&self, url: String, url_composer: Url) -> bool {
        if url.is_empty() {
            return false;
        }

        self.get_element (&mut url, &mut url_composer.frag_, '#');
        self.get_element (&mut url, &mut url_composer.query_, '?');
        self.get_element (&mut url, &mut url_composer.param_, ';');

        let pos =  match url.find(':') {
            Some (p) => p,
            None => 0,
        };

        if 0 != pos {
            url_composer.scheme_ = url[..pos].to_lowercase();
            url = url[pos+1..].to_string();
        }

        let pos = match url.find("//") {
            Some (p) => p,
            None => 9999,
        };

        if 0 == pos {
            url = url[2..].to_string();
            let pos = match url.find ('/') {
                Some (p) => p,
                None => url.len(),
            };
            url_composer.net_loc_ = url[..pos].to_string();
            if pos < url.len() {
                url = url[pos+1..].to_string();
            }
            else {
                url = "".to_string();
            }
            let pos = match url_composer.net_loc_.find (':') {
                Some (p) => p,
                None => 0,
            };
            if 0 != pos {
                url_composer.port_ = url_composer.net_loc_[pos+1..].parse().unwrap();
                url_composer.net_loc_ = url_composer.net_loc_[..pos].to_string();
            }
        }

        url_composer.path_ = url;

        url_composer.net_loc_ = url_composer.net_loc_.to_lowercase ();
        if url_composer.path_.is_empty() {
            url_composer.path_ = "/".to_string();
        }

        true
    }
}