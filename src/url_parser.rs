
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
    fn get_url (range: Range) => String {
        let mut url = String::new("");
        if (range & Range::SCHEME && !scheme_.is_empty())
            url = scheme_ + ":";
        if (range & Range::NETLOC && !net_loc_.is_empty())
            url += "//" + net_loc_;
        if (range & Range::NETLOC && port_ != DEFAULT_PORT)
            url += ":" + port_.to_string();
        if (range & Range::PATH) {
            url += path_;
            let path_tmp = path_.clone();
            if (path_.len() > 1 && path_tmp.pop() == '/' && path_.find('.') != None)
                url.truncate (url.rfind ('/').unwrap());
        }
        if (range & Range::PARAM && !param_.is_empty())
            url += ";" + param_;
        if (range & Range::QUERY && !query_.is_empty())
            url += "?" + query_;
        if (range & Range::FRAGMENT && !frag_.is_empty())
            url += "#" + frag_;
        url
    }
}