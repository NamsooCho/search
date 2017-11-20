use std::collections::{VecDeque, BTreeSet};
//use url_parser::Range;
use url::Url;

#[derive(Debug, Clone)]
pub struct SyncQ {
    url: VecDeque<Box<Url>>,
    url_history: BTreeSet<Box<Url>>,
    limit_: u32
}
/*
impl Copy for Option<::url::Url> {}
impl Clone for Option<::url::Url> {
    fn clone(&self) -> Option<::url::Url> {
        *self
    }
}
*/
impl SyncQ {
    pub fn new (seed: &String, limit: u32) -> SyncQ {
        let mut s = SyncQ {
            url: VecDeque::new(), 
            url_history: BTreeSet::new(),
            limit_: 0
        };
        if let Ok(u) = Url::parse(&seed.clone()) {
            let url  = Box::new(u);
            s.url_history.insert(url.clone());
            s.url.push_back(url);
            s.limit_ = limit;
        }
        s
    }

    pub fn full (&self) -> bool {
        self.url.len() as u32 > self.limit_
    }

    pub fn get_next_url (&mut self) -> Option<Box<Url>> {
        match self.url.pop_front() {
            Some(x) => Some(x),
            None    => None,
        }
    }

    pub fn insert (&mut self,  base_url: &mut Url, url_list: & Vec<String>) {
        for elem in url_list.iter() {
            let new_url = match Url::parse(&elem) {
                Ok(u) => Box::new(u),
                Err(_) => match base_url.join(&elem) {
                    Ok(u) => Box::new(u),
                    Err(_) => {continue;},
                },
            };

            if new_url.scheme() != "http" && new_url.scheme() != "https" {
                continue;
            }

            if !self.url_history.contains(&new_url) {
                self.url_history.insert(new_url.clone());
                self.url.push_back(new_url);
            }
        }
    }
}