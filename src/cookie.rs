use url_parser::Url;

pub struct Cookie {
    value_: String,
    expires_: String,
    path_: String,
    domain_: String,
    secure_: String,
}

impl Cookie {

    fn search_cookie_value (&self, cookie: &String, field: &String) -> String {
        let mut b_pos = 0;
        let mut e_pos = 0;

        b_pos = match cookie.find (field) {
            None => 0,
            Some(p) => p,
        };

        let cookie_bak = cookie.clone();

        if b_pos != 0 {
            let cookie = cookie[b_pos+field.len()..].to_string();
            b_pos = match cookie.find ('=') {
                None => 0,
                Some(p) => p,
            };
            if b_pos != 0 {
                let cookie = cookie[b_pos+1..].to_string();
                e_pos = match cookie.find (';') {
                    None => 0,
                    Some(p) => p,
                };
                if e_pos != 0 {
                    return cookie.to_string();
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

        let mut cookie_info = Cookie {
            value_: "".to_string(),
            expires_: "".to_string(),
            path_: "".to_string(),
            domain_: "".to_string(),
            secure_: "".to_string(),
        };
        let name = cookie[a..a+b].to_string();
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

    }
}