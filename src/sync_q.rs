use std::collections::{VecDeque, BTreeSet};
use url_parser::{MyUrl,Range};
//use num::FromPrimitive;

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
//        let mut parser = MyUrl::new();
        let mut url = MyUrl::new();
        url.parse(&seed.clone());
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
            base_url.parse(&elem);
            if !base_url.filter() {
                continue;
            }

            if !self.url_history.contains(&base_url) {
                self.url_history.insert(base_url.clone());
                self.url.push_back(base_url.clone());
            }
        }
    }
}