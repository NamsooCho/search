use std::path::Path;
use std::fs::File;
use std::net::TcpStream;
use std::error::Error;
use std::collections::{BTreeSet};

use sync_q::SyncQ;
use url_parser::Url;

struct DNS {

}

#[derive(Debug, Clone)]
pub struct HttpSocketThread {
    continue_: bool,
    url_q: SyncQ,
    output_: String,
    redir_history: BTreeSet<Url>,
}

impl DNS {
    fn get_sock_addr (host: &str, port: &str) {}
}

impl HttpSocketThread {
    pub fn new () -> HttpSocketThread {
        let mut sock = HttpSocketThread{continue_: true, url_q: SyncQ::new(), output_: "".to_string(), redir_history: BTreeSet::new()};
        sock
    }

    fn check_redir (&mut self, url: &Url) -> bool {
        if self.redir_history.contains(url) {
            return false;
        } else {
            self.redir_history.insert (url.clone());
        }
        true
    }

    fn make_http_header (&self, url: &str, host: &str, cookie: &str) -> String {
        let mut hdr: String = "GET ".to_string();
        if !url.is_empty() {
            hdr = hdr + url;
        }
        else {
            hdr += "/";
        }
        hdr = hdr + " HTTP/1.1\r\n";
        hdr = hdr + "Host: " + host + "\r\n";
        hdr = hdr + "User-Agent: TinyCrawler\r\n";
        hdr = hdr + "Accept: */*\r\n";
        hdr = hdr + "Accept-Language: ko\r\n";
        hdr = hdr + "Cookie: " + cookie + "\r\n";
        hdr = hdr + "\r\n";
        hdr
    }

    fn request (&mut self, url: &Url) -> bool {
        if url.empty() {
            return false;
        }

        self.redir_history.clear ();
        let err_: String = "".to_string();
        let done: bool = false;
        let ret: bool = false;

        while !done && err_.is_empty() {
            if !self.check_redir(&url) {
                break;
            }

            let ip: &str = &url.net_loc_[..];
            let mut tcp_s = TcpStream::connect ((ip, url.port_));
            let send_data = self.make_http_header ("a", "b", "c");
        }
        true
    }

    fn thread_function (&mut self) {
        //let html_parser: HtmlParse = HtmlParser::new();

        let mut html_cnt = 0;

        while self.continue_ {
            if self.url_q.full () {
                break;
            }
            
            let url: Url = self.url_q.get_next_url ();
            if self.request (&url) {
                self.output_ = self.output_.clone() + &html_cnt.to_string() + ".html";
                html_cnt = html_cnt + 1;
                let out_path = Path::new(&self.output_);
                let display = out_path.display();
                let mut out_file = match File::create(&out_path) {
                    Ok(f) => f,
                    Err(why) => panic!("couldn't open {}: {}", display, why.description()),
                };
                //out_file.write_all (self.get_body ().as_bytes()).unwrap();
                //html_parser.parse (self.get_body().to_string(), self.get_body().to_string().len());
                //self.url_q.insert (&url, html_parser.extract_link_url_list ());
            }
            else {
                //error!("{} --> {}", url.url, self.get_err_msg());
            }
        }
    }

    pub fn initiate (&mut self) {
        self.continue_ = true;
        self.redir_history = BTreeSet::new();
        self.thread_function ();
    }
}