use std::path::Path;
use std::fs::File;
use std::net::TcpStream;
use std::error::Error;
use std::collections::{BTreeSet};
use std::io::prelude::*;
//use std::net::{SocketAddrV4,Ipv4Addr};
use openssl::ssl::{SslMethod, SslConnectorBuilder, SslStream};
use std::sync::{Arc,Mutex};
use std::time::Duration;
use std::thread;

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
    url_q: Arc<Mutex<SyncQ>>,
    output_: String,
    redir_history: BTreeSet<Url>,
    cookie_: Arc<Mutex<Cookie>>,
    dns_: Dns,
    http_parser_: HttpParser,
    err_: String,
    out_dir_: String,
    thread_idx: i32
}

impl HttpSocketThread {
    pub fn new (out_dir: &String) -> HttpSocketThread {
        HttpSocketThread {
            continue_: true, 
            url_q: Arc::new(Mutex::new(SyncQ::new(&"".to_string(), 1000))), 
            output_: "".to_string(), 
            redir_history: BTreeSet::new(), 
            cookie_: Arc::new(Mutex::new(Cookie::new())),
            dns_: Dns::new(),
            http_parser_: HttpParser::new(),
            err_: String::new(),
            out_dir_: out_dir.clone(),
            thread_idx: 0
        }
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
        //let mut ret = 0;
        let mut recv_size;
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

    fn recv_data_ssl (&mut self, sock: &mut SslStream<TcpStream>) -> bool {
        let mut data = Vec::new();
        //let mut ret = 0;
        let mut recv_size;
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
        //let mut ret: bool = false;

        while !done && err_.is_empty() {
            if !self.check_redir(&url) {
                break;
            }

            let addr;
            match self.dns_.get_sock_addr (&url.get_net_loc()) {
                Some(e) => {
                    addr = e;
                    let ip = addr.ip().octets();
                    let mut port = 80;
                    let connector = SslConnectorBuilder::new(SslMethod::tls()).unwrap().build();

                    let send_data;
                    let cook = self.cookie_.lock().unwrap().get_cookie(url);
                    send_data = self.make_http_header (
                        url.get_url_str(Range::PATH as u8|Range::PARAM as u8|Range::QUERY as u8), 
                        url.get_net_loc(), cook);

                    let mut tcp_s = match TcpStream::connect (format!("{}.{}.{}.{}:{}",ip[0],ip[1],ip[2],ip[3],port))  {
                        Ok(s) => s,
                        _ => {
                            err_ = format!("\"{}.{}.{}.{}:{}\"",ip[0],ip[1],ip[2],ip[3],port).to_string();
                            continue;
                        },
                    };

                    if url.get_scheme() == "https" {
                        port = 443;
                        tcp_s = match TcpStream::connect (format!("{}.{}.{}.{}:{}",ip[0],ip[1],ip[2],ip[3],port))  {
                            Ok(s) => s,
                            _ => {
                                err_ = format!("\"{}.{}.{}.{}:{}\"",ip[0],ip[1],ip[2],ip[3],port).to_string();
                                continue;
                            },
                        };
                        let mut tcp_ssl = connector.connect(&url.get_net_loc(), tcp_s).unwrap();
                        match tcp_ssl.write(send_data.as_bytes())
                        {
                            Ok(_) => {;},
                            Err(e) => { error! ("Tcp ssl write error {}", e)},
                        };
                        self.recv_data_ssl (&mut tcp_ssl);

                    }
                    else {
                        match tcp_s.write(send_data.as_bytes())
                        {
                            Ok(_) => {;},
                            Err(e) => { error! ("Tcp write error {}", e)},
                        };
                        self.recv_data (&mut tcp_s);
                    }

                    self.cookie_.lock().unwrap().insert(&self.http_parser_.get_cookie(), &url);
                },
                None => {},
            };

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
            if self.url_q.lock().unwrap().full () {
                break;
            }
            
            let mut url: Url = self.url_q.lock().unwrap().get_next_url ();
            if url.empty() {
                thread::sleep(Duration::from_secs(3));
                continue;
            }

            if self.request (&mut url) {
                self.output_ = self.out_dir_.clone() + &self.thread_idx.to_string() + "_" + &html_cnt.to_string() + ".html";
                html_cnt = html_cnt + 1;
                let out_path = Path::new(&self.output_);
                let display = out_path.display();
                let mut out_file = match File::create(&out_path) {
                    Ok(f) => f,
                    Err(why) => panic!("couldn't open {}: {}", display, why.description()),
                };
                out_file.write_all (self.http_parser_.get_body ().as_bytes()).unwrap();
                html_parser.parse (self.http_parser_.get_body().to_string());
                self.url_q.lock().unwrap().insert (&mut url, &mut html_parser.extract_link_url_list ());
            }
            else {
                error!("{} --> {}", url.get_url_str(0xFF), self.err_);
            }
        }
    }

    pub fn initiate (&mut self, idx: i32, queue: Arc<Mutex<SyncQ>>, cookie: Arc<Mutex<Cookie>>) {
        self.continue_ = true;
        self.redir_history = BTreeSet::new();
        self.url_q = queue;
        self.cookie_ = cookie;
        self.thread_idx = idx;
        self.thread_function ();
    }
}