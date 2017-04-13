use std::path::Path;
use std::fs::File;
use std::net::TcpStream;
use std::error::Error;
use std::collections::{BTreeSet};
use std::io::prelude::*;
use std::net::{SocketAddr,SocketAddrV4,Ipv4Addr};

use sync_q::SyncQ;
use url_parser::Url;
use cookie::Cookie;
use html_parser::HtmlParser;
use http_parser::HttpParser;
use dns::Dns;
use url_parser::Range;

#[derive(Debug, Clone)]
pub struct HttpSocketThread {
    continue_: bool,
    url_q: SyncQ,
    output_: String,
    redir_history: BTreeSet<Url>,
    cookie_: Cookie,
    dns_: Dns,
    http_parser_: HttpParser,
    err_: String,
}

impl HttpSocketThread {
    pub fn new () -> HttpSocketThread {
        let mut sock = HttpSocketThread {
            continue_: true, 
            url_q: SyncQ::new(), 
            output_: "".to_string(), 
            redir_history: BTreeSet::new(), 
            cookie_: Cookie::new(),
            dns_: Dns::new(),
            http_parser_: HttpParser::new(),
            err_: String::new(),
        };
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

    fn make_http_header (&self, url: String, host: String, cookie: String) -> String {
        let mut hdr: String = "GET ".to_string();
        if !url.is_empty() {
            hdr = hdr + &url;
        }
        else {
            hdr += "/";
        }
        hdr = hdr + " HTTP/1.1\r\n";
        hdr = hdr + "Host: " + &host + "\r\n";
        hdr = hdr + "User-Agent: TinyCrawler\r\n";
        hdr = hdr + "Accept: */*\r\n";
        hdr = hdr + "Accept-Language: ko\r\n";
        hdr = hdr + "Cookie: " + &cookie + "\r\n";
        hdr = hdr + "\r\n";
        hdr
    }

    fn recv_data (&mut self, sock: &mut TcpStream) -> bool {
        let mut data = Vec::new();
        let mut ret = 0;
        let mut recv_size = 0;
        let mut done = false;

        self.http_parser_.clear();

        while !done {
            recv_size = sock.read_to_end (&mut data).unwrap();
            if recv_size <= 0 {
                self.err_ = "connection closed.".to_string();
                return false;
            }

            self.http_parser_.parse(&mut data);
            done = !self.http_parser_.is_partial();
            data.clear();
        }

        if !self.http_parser_.is_ok() && !self.http_parser_.is_redirect() {
            self.err_ = format!("HTTP Error (Response Code: {})", self.http_parser_.get_rep_code());
        }

        self.http_parser_.is_ok()
    }

    fn request (&mut self, url: &mut Url) -> bool {
        if url.empty() {
            return false;
        }

        self.redir_history.clear ();
        let mut err_: String = "".to_string();
        let mut done: bool = false;
        let mut ret: bool = false;

        while !done && err_.is_empty() {
            if !self.check_redir(&url) {
                break;
            }

            let mut addr = SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 80);
            if self.dns_.get_sock_addr (&url.get_net_loc(), &mut addr) {
                let mut tcp_s = match TcpStream::connect (addr) {
                    Ok(s) => s,
                    _ => {
                        err_ = "fail to connect".to_string();
                        continue;
                    },
                };
                let mut send_data = String::new();
                let cook = self.cookie_.get_cookie(url);
                send_data = self.make_http_header (
                    url.get_url_str(Range::PATH as u8|Range::PARAM as u8|Range::QUERY as u8), 
                    url.get_net_loc(), cook);
                tcp_s.write(send_data.as_bytes());
                self.recv_data (&mut tcp_s);
                self.cookie_.insert(&self.http_parser_.get_cookie(), &url);
            }
            if self.http_parser_.is_redirect() && !self.http_parser_.get_location().is_empty() {
                url.update(self.http_parser_.get_location());
            }
            else {
                done = true;
            }
        }
        true
    }

    fn thread_function (&mut self) {
        let mut html_parser: HtmlParser = HtmlParser::new();

        let mut html_cnt = 0;

        while self.continue_ {
            if self.url_q.full () {
                break;
            }
            
            let mut url: Url = self.url_q.get_next_url ();
            if self.request (&mut url) {
                self.output_ = self.output_.clone() + &html_cnt.to_string() + ".html";
                html_cnt = html_cnt + 1;
                let out_path = Path::new(&self.output_);
                let display = out_path.display();
                let mut out_file = match File::create(&out_path) {
                    Ok(f) => f,
                    Err(why) => panic!("couldn't open {}: {}", display, why.description()),
                };
                out_file.write_all (self.http_parser_.get_body ().as_bytes()).unwrap();
                html_parser.parse (self.http_parser_.get_body().to_string());
                self.url_q.insert (&mut url, &mut html_parser.extract_link_url_list ());
            }
            else {
                error!("{} --> {}", url.get_url_str(0xFF), self.err_);
            }
        }
    }

    pub fn initiate (&mut self, queue: &mut SyncQ, cookie: &mut Cookie) {
        self.continue_ = true;
        self.redir_history = BTreeSet::new();
        self.thread_function ();
        self.url_q = queue.clone();
        self.cookie_ = cookie.clone();
    }
}