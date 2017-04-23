use std::collections::{VecDeque, BTreeSet};
use url_parser::{Url, Range};
use num::FromPrimitive;

#[derive(Debug, Clone)]
pub struct SyncQ {
    url: VecDeque<Url>,
    url_history: BTreeSet<Url>,
    limit_: u32
}

impl SyncQ {
    pub fn new (seed: &String, limit: u32) -> SyncQ {
        let mut s = SyncQ {
            url: VecDeque::new(), 
            url_history: BTreeSet::new(),
            limit_: 0
        };
        let parser = Url::new();
        let mut url = Url::new();
        parser.parse(&mut seed.clone(), &mut url);
        s.url_history.insert(url.clone());
        s.url.push_back(url);
        s.limit_ = limit;
        s
    }

    pub fn full (&self) -> bool {
        self.url.len() as u32 > self.limit_
    }

    pub fn get_next_url (&mut self) -> Url {
        let u = match self.url.pop_front() {
            Some(x) => x,
            None    => Url::new(),
        };
        u
    }

    pub fn insert (&mut self,  base_url: &mut Url, url_list: &mut Vec<String>) {
        for mut elem in url_list.iter_mut() {
            let mut url: Url = Url::new();
            base_url.parse(&mut elem, &mut url);
            base_url.get_abs_path(&url);
            if !url.filter() {
                continue;
            }

            let mut temp: Url = Url::new();
            let range = match Range::from_u8(Range::SCHEME as u8 | Range::NETLOC as u8 | Range::PATH as u8) {
                Some(x) => x,
                None => Range::NONE
            };

            url.parse(&mut url.get_url(range), &mut temp);
            if !self.url_history.contains(&temp) {
                self.url_history.insert(temp.clone());
                self.url.push_back(temp.clone());
            }
        }
    }
}