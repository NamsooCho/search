use url_parser::Url;

#[derive(Debug, Clone)]
pub struct SyncQ {
    url: Vec<Url>,
    index: usize,
}

impl SyncQ {
    pub fn new () -> SyncQ {
        SyncQ{url: Vec::new(), index: 0}
    }

    pub fn full (&self) -> bool {
        true
    }

    pub fn get_next_url (&mut self) -> Url {
        let u = self.url.last().cloned().unwrap();
        self.url.pop();
        u
    }
}