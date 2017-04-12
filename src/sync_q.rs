use std::collections::{VecDeque, BTreeSet};
use url_parser::{Url, Range};
use num::FromPrimitive;

#[derive(Debug, Clone)]
pub struct SyncQ {
    url: VecDeque<Url>,
    url_history: BTreeSet<Url>,
}

impl SyncQ {
    pub fn new () -> SyncQ {
        SyncQ{url: VecDeque::new(), url_history: BTreeSet::new()}
    }

    pub fn full (&self) -> bool {
        true
    }

    pub fn get_next_url (&mut self) -> Url {
        self.url.pop_front().unwrap()
    }

    pub fn insert (&mut self,  base_url: &mut Url, urlList: &Vec<String>) {
        for elem in urlList.iter() {
            let url: Url;
            base_url.parse(&mut elem, &mut url);
            base_url.get_abs_path(&url);
            if !url.filter() {
                continue;
            }

            let mut temp: Url;
            url.parse(&mut url.get_url(Range::from_u8(Range::SCHEME as u8 | Range::NETLOC as u8 | Range::PATH as u8).unwrap()), &mut temp);
            if !self.url_history.contains(&temp) {
                self.url_history.insert(temp.clone());
                self.url.push_back(temp.clone());
            }
        }
    }
}