use std::collections::{VecDeque, BTreeSet};
use url_parser::{MyUrl};
//use num::FromPrimitive;

bitflags! {
    flags Range: u8 {
        const SCHEME = 0x01,
        const NETLOC = 0x02,
        const PATH = 0x04,
        const PARAM = 0x08,
        const QUERY = 0x10,
        const FRAGMENT = 0x20,
        const ALL = 0xFF,
        const NONE = 0x00,
        const SCHEME_NETLOC_PATH = 0x07,
    }
}

#[derive(Debug, Clone)]
pub struct SyncQ {
    url: VecDeque<MyUrl>,
    url_history: BTreeSet<MyUrl>,
    limit_: u32
}

impl SyncQ {
    pub fn new (seed: &String, limit: u32) -> SyncQ {
        let mut s = SyncQ {
            url: VecDeque::new(), 
            url_history: BTreeSet::new(),
            limit_: 0
        };
        let mut parser = MyUrl::new();
        let url = MyUrl::new();
        parser.parse(&seed.clone());
        s.url_history.insert(url.clone());
        s.url.push_back(url);
        s.limit_ = limit;
        s
    }

    pub fn full (&self) -> bool {
        self.url.len() as u32 > self.limit_
    }

    pub fn get_next_url (&mut self) -> MyUrl {
        let u = match self.url.pop_front() {
            Some(x) => x,
            None    => MyUrl::new(),
        };
        u
    }

    pub fn insert (&mut self,  base_url: &mut MyUrl, url_list: &mut Vec<String>) {
        for elem in url_list.iter_mut() {
            let mut url: MyUrl = MyUrl::new();
            base_url.parse(&elem);
            base_url.get_abs_path(&mut url);
            if !url.filter() {
                continue;
            }

            let temp: MyUrl = MyUrl::new();
            let url_str = url.get_url_str(SCHEME.bits | NETLOC.bits | PATH.bits);
            url.parse(&url_str);
            if !self.url_history.contains(&temp) {
                self.url_history.insert(temp.clone());
                self.url.push_back(temp.clone());
            }
        }
    }
}