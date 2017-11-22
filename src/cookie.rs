use multimap::MultiMap;
use url::Url;
use time;

#[derive(Debug, Clone, PartialOrd,Ord,PartialEq,Eq)]
pub struct StCookie {
    pub value_: String,
    pub expires_: String,
    pub path_: String,
    pub domain_: String,
    pub secure_: String,
}

#[derive(PartialEq,Eq,Debug,Clone)]
pub struct Cookie {
    pub cookie_:  Box<MultiMap<String, StCookie>>
}

impl Cookie {
    pub fn new() -> Cookie {
        Cookie {
            cookie_: Box::new(MultiMap::new()),
        }
    }

    pub fn insert (&mut self, cookie_arr: &Vec<String>, url: &mut Box<Url>) {
        for c in cookie_arr.iter() {
            self.parse (&c, url);
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

    pub fn get_cookie (&mut self, url: &Url) -> String {
        let mut result = String::new();

        for (key, value) in self.cookie_.clone().iter_mut() {
            if (url.host_str().unwrap() == value.domain_ || url.host_str().unwrap().contains(&value.domain_))
                && !url.path().contains(&value.path_) {
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
            let pos = result.rfind(";").unwrap();
            result.remove(pos);
        }
        result
    }

    fn search_cookie_value (&self, cookie: &String, field: &String) -> String {
        let mut b_pos;
        let e_pos;

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

    pub fn parse (&mut self, cookie: &String, url: &mut Box<Url>) {
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
        let name = cookie[..b].to_string();
        cookie_info.value_ = self.search_cookie_value(&cookie, &name);
        cookie_info.expires_ = self.search_cookie_value(&cookie, &"expires".to_string());
        cookie_info.path_ = self.search_cookie_value(&cookie, &"path".to_string());
        cookie_info.domain_ = self.search_cookie_value(&cookie, &"domain".to_string());
        cookie_info.secure_ = self.search_cookie_value(&cookie, &"secure".to_string());

        if cookie_info.path_.is_empty() {
            let mut path = url.path().to_string();
            if path.rfind ('.') != None {
                let end_pos = path.rfind ('/').unwrap ();
                path.drain (..end_pos);
            }
            cookie_info.path_ = path;
        }

        if cookie_info.domain_.is_empty() {
            cookie_info.domain_ = url.host_str().unwrap().to_string();
        }

        if !self.cookie_.contains_key(&name) {
            self.cookie_.insert (name, cookie_info);
        }
        else {
            let g = self.cookie_.get_vec_mut(&name).unwrap ();
            for v in g.iter_mut() {
                if v.path_ == cookie_info.path_ && &v.domain_ == &cookie_info.domain_ {
                    v.value_ = cookie_info.value_;
                    break;
                }
            }
        }
    }
}
