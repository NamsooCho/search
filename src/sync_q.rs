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

    pub fn insert (&mut self,  base_url: &mut Url, urlList: &Vec<Url>) {
        for elem in urlList.iter() {
            let url: Url = base_url.get_abs_path(elem);
            //if !filter(url) {
            //    continue;
            //}

            let temp: Url = url.get_url(Range::from_u8(Range::SCHEME as u8 | Range::NETLOC as u8 | Range::PATH as u8).unwrap());
            if !self.url_history.contains(&temp) {
                self.url_history.insert(temp.clone());
                self.url.push_back(temp.clone());
            }
        }
    }
}