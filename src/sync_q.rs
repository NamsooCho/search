use url_parser::Url;

pub struct SyncQ {
    url: Url,
}

impl SyncQ {
    pub fn new () -> SyncQ {
        let mut q = SyncQ{url: Url::new()};
        q
    }

    pub fn full (&self) -> bool {
        true
    }

    pub fn get_next_url (&self) -> Url {
        self.url
    }
}