use std::collections::{VecDeque, BTreeSet};
//use url_parser::Range;
use url::Url;

#[derive(Debug, Clone)]
pub struct SyncQ {
    url: VecDeque<Option<Url>>,
    url_history: BTreeSet<Option<Url>>,
    limit_: u32
}

impl SyncQ {
    pub fn new (seed: &String, limit: u32) -> SyncQ {
        let mut s = SyncQ {
            url: VecDeque::new(), 
            url_history: BTreeSet::new(),
            limit_: 0
        };
        let url = match Url::parse(&seed.clone()) {
            Ok(u) => Some(u),
            Err(_) =>None,
        };
        s.url_history.insert(url.clone());
        s.url.push_back(url);
        s.limit_ = limit;
        s
    }

    pub fn full (&self) -> bool {
        self.url.len() as u32 > self.limit_
    }

    pub fn get_next_url (&mut self) -> Option<Url> {
        match self.url.pop_front() {
            Some(x) => x,
            None    => None,
        }
    }

    pub fn insert (&mut self,  base_url: &mut Url, url_list: & Vec<String>) {
        for elem in url_list.iter() {
            let new_url = match base_url.join(&elem) {
                Ok(u) => Some(u),
                Err(_) => None,
            };

            if new_url == None {
                continue;
            }
            else {
                if new_url.clone().unwrap().scheme() != "http" && new_url.clone().unwrap().scheme() != "https" {
                    continue;
                }
            }

            if !self.url_history.contains(&new_url) {
                self.url_history.insert(new_url.clone());
                self.url.push_back(new_url.clone());
            }
        }
    }
}