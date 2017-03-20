use multimap::MultiMap;
use cookie::Cookie;

pub struct CookieContainer {
    cookie_container: MultiMap<String,Cookie>,
}

impl CookieContainer {
    pub fn new () -> CookieContainer {
        let mut s = CookieContainer {cookie_container: MultiMap::new()};
        s
    }


}