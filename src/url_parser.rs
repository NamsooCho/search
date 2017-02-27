
const DEFAULT_PORT: i16 = 80;

enum Range {
    SCHEME = 0x01,
    NETLOC = 0x02,
    PATH = 0x04,
    PARAM = 0x08,
    QUERY = 0x10,
    FRAGMENT = 0x20,
    ALL = 0xFF,
}

struct Url {
    scheme_: String,
    net_loc_: String,
    path_: String,
    param_: String,
    query_: String,
    frag_: String,
    port_: i16,
}

impl Url {
    fn get_url (range: Range) -> String {
        let mut url = String::new("");
        if (range & Range::SCHEME && !scheme_.is_empty()) {
            url = scheme_ + ":";
        }
        if (range & Range::NETLOC && !net_loc_.is_empty()) {
            url += "//" + net_loc_;
        }
        if (range & Range::NETLOC && port_ != DEFAULT_PORT) {
            url += ":" + port_.to_string();
        }
        if (range & Range::PATH) {
            url += path_;
            let path_tmp = path_.clone();
            if (path_.len() > 1 && path_tmp.pop() == '/' && path_.find('.') != None) {
                url.truncate (url.rfind ('/').unwrap());
            }
        }
        if (range & Range::PARAM && !param_.is_empty()) {
            url += ";" + param_;
        }
        if (range & Range::QUERY && !query_.is_empty()) {
            url += "?" + query_;
        }
        if (range & Range::FRAGMENT && !frag_.is_empty()) {
            url += "#" + frag_;
        }
        url
    }

    fn swap (other: Url) {
        std::mem::swap (&mut self, &mut other);
    }

    fn get_element (url: &mut String, element: &String, c: char) {
        let pos = match url.find(c) {
            Some(p) => p,
            None => 0,
        };

        if (0 != pos) {
            *element = url[pos+1..];
            url.truncate(pos);
        }
    }

    fn parse (url: String, url_composer: Url) -> bool {
        if (url.is_empty()) {
            return false;
        }

        get_element (&mut url, url_composer.frag_, '#');
        get_element (&mut url, url_composer.query_, '?');
        get_element (&mut url, url_composer.param_, ';');

        let pos =  match url.find(':') {
            Some (p) => p,
            None => 0,
        };

        if (0 != pos) {
            url_composer.scheme_ = url[..pos].to_lowercase();
            url = url[pos+1..];
        }

        let pos = match url.find("//") {
            Some (p) => p,
            None => 9999,
        };

        if (0 == pos) {
            url = url[2..];
            let pos = match url.find ('/') {
                Some (p) => p,
                None => url.len(),
            };
            url_composer.net_loc_ = url[..pos];
            if (pos < url.len()) {
                url = url[pos+1..];
            }
            else {
                url.erase();
            }
            let pos = match url_composer.net_loc_.find (':') {
                Some (p) => p,
                None => 0,
            };
            if (0 != pos) {
                url_composer.port_ = url_composer.net_loc_[pos+1..].parse().unwrap();
                url_composer.net_loc_ = url_composer.net_loc_[..pos];
            }
        }

        url_composer.path_ = url;

        url_composer.net_loc_ = url_composer.net_loc_.to_lowercase ();
        if (url_composer.path_.is_empty()) {
            url_composer.path_ = "/";
        }

        true
    }
}