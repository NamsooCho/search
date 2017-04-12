use multimap::MultiMap;
use url_parser::Url;
use std::fmt;
use time;

#[derive(Debug, Clone, PartialOrd,Ord,PartialEq,Eq)]
pub struct StCookie {
    pub value_: String,
    pub expires_: String,
    pub path_: String,
    pub domain_: String,
    pub secure_: String,
}

#[derive(PartialEq,Eq)]
pub struct Cookie {
    pub cookie_:  MultiMap<String, StCookie>
}

impl Clone for Cookie {
    fn clone(&self) -> Cookie { *self }
}

impl fmt::Debug for Cookie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cookie: {:?}", self)
    }
}

impl Cookie {
    pub fn new() -> Cookie {
        let mut c = Cookie {
            cookie_: MultiMap::new(),
        };
        c
    }

    pub fn clear (&self) {
        
    }

    pub fn insert (&self, cookie_arr: &Vec<String>, url: &Url) {
        for c in cookie_arr.iter_mut() {
            self.parse (&c, &url);
        }
    }

    fn is_expired (&self, date: &String) -> bool {
        if date.is_empty() {
            return false;
        }

        let expire = time::strptime (&date, "%FT%T%z").unwrap();
        let now = time::now();

        expire < now
    }

    pub fn get_cookie (&self, url: &Url) -> String {
        let mut result = String::new();

        if url.empty() {
            return result;
        }

        for (key, value) in self.cookie_.iter_mut() {
            if (url.compare_netloc(&value.domain_, &url.get_net_loc()) || !(&url.get_net_loc()).contains(&value.domain_)) 
                && !(&url.get_path()).contains(&value.path_) {
                if value.expires_.is_empty() || !self.is_expired (&value.expires_) {
                    result = result + &key + &"=" + &value.value_ + &"; ";
                }
                else {
                    self.cookie_.remove(key);
                    continue;
                }
            }
        }
        if !result.is_empty() {
            result.remove(result.rfind(";").unwrap());
        }
        result
    }

    fn search_cookie_value (&self, cookie: &String, field: &String) -> String {
        let mut b_pos = 0;
        let mut e_pos = 0;

        b_pos = match (&cookie).find (&*field) {
            Some(p) => p,
            None => 0,
        };

        let cookie_bak = cookie.clone();

        if b_pos != 0 {
            let cookie_ = cookie[b_pos+field.len()..].to_string();
            b_pos = match (&cookie_).find ('=') {
                None => 0,
                Some(p) => p,
            };
            if b_pos != 0 {
                let cookie_ = cookie[b_pos+1..].to_string();
                e_pos = match (&cookie_).find (';') {
                    None => 0,
                    Some(p) => p,
                };
                if e_pos != 0 {
                    return cookie_.to_string();
                }
                else {
                    return cookie_bak[b_pos..].to_string();
                }
            }
        }
        return "".to_string();
    }

    pub fn parse (&self, cookie: &String, url: &Url) {
        if cookie.is_empty() {
            return;
        }

        let a = match cookie.find (';') {
            None => 0,
            Some(p) => p,
        };
        let b = match cookie.find ('=') {
            None => 0,
            Some(p) => p,
        };
        if a < b || b == 0 {
            return;
        }

        let mut cookie_info = StCookie {
            value_: "".to_string(),
            expires_: "".to_string(),
            path_: "".to_string(),
            domain_: "".to_string(),
            secure_: "".to_string(),
        };
        let mut name = cookie[a..a+b].to_string();
        cookie_info.value_ = self.search_cookie_value(&cookie, &name);
        cookie_info.expires_ = self.search_cookie_value(&cookie, &"expires".to_string());
        cookie_info.path_ = self.search_cookie_value(&cookie, &"path".to_string());
        cookie_info.domain_ = self.search_cookie_value(&cookie, &"domain".to_string());
        cookie_info.secure_ = self.search_cookie_value(&cookie, &"secure".to_string());

        if cookie_info.path_.is_empty() {
            let mut path = url.get_path();
            if path.rfind ('.') != None {
                let end_pos = path.rfind ('/').unwrap ();
                path.drain (..end_pos);
            }
            cookie_info.path_ = path;
        }

        if cookie_info.domain_.is_empty() {
            cookie_info.domain_ = url.get_net_loc ();
        }

        if !self.cookie_.contains_key(&name) {
            self.cookie_.insert (name, cookie_info);
        }
        else {
            let mut g = self.cookie_.get_vec(&name).unwrap ();
            for v in &mut g.iter_mut() {
                if v.path_ == cookie_info.path_ && url.compare_netloc (&v.domain_, &cookie_info.domain_) {
                    v.value_ = cookie_info.value_;
                    break;
                }
            }
        }
    }
}
