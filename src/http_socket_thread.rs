use std::path::Path;
use std::fs::File;
use std::net::TcpStream;
use std::error::Error;
use std::collections::{BTreeSet};
use std::io::prelude::*;
use openssl::ssl::{SslMethod, SslConnectorBuilder, SslStream};
use std::sync::{Arc,Mutex};
use std::time::Duration;
use std::thread;

use sync_q::SyncQ;
use url::Url;
use cookie::Cookie;
use html_parser::HtmlParser;
use http_parser::HttpParser;
use dns::Dns;

trait Socketable { 
    fn read_to_end_gen (&mut self, data: &mut Vec<u8>) -> Result<usize, ::std::io::Error>;
}
impl Socketable for SslStream<TcpStream> {
    fn read_to_end_gen (&mut self, data: &mut Vec<u8>) -> Result<usize, ::std::io::Error> {
        self.read_to_end(data)
    }
}
impl Socketable for TcpStream {
    fn read_to_end_gen (&mut self, data: &mut Vec<u8>) -> Result<usize, ::std::io::Error> {
        self.read_to_end(data)
    }
}

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
            url_q: Arc::new(Mutex::new(SyncQ::new(&"".to_string(), 10000))), 
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

    fn check_redir (&mut self, url: &mut Box<Url>) -> bool {
        if self.redir_history.contains(url) {
            return false;
        } else {
            self.redir_history.insert (*url.clone());
        }
        true
    }

    fn make_http_header (&self, url: &mut Box<Url>, cookie: String) -> String {
        let host = match url.host_str() {
            Some(h) => h,
            None => { panic!("url broken"); },
        };
        let path = url.path();
        let query = match url.query() {
            Some(q) => "?".to_string() + q,
            None => "".to_string(),
        };
        
        let mut hdr: String = "GET ".to_string();
        
        if path.is_empty() {
            hdr += "/";
        }
        else {
            hdr = hdr + path;
            if !query.is_empty() {
                hdr = hdr + &query;
            }
        }
        hdr = hdr + " HTTP/1.1\r\n";
        hdr = hdr + "Host: " + host + "\r\n";
        hdr = hdr + "User-Agent: TinyCrawler\r\n";
        hdr = hdr + "Accept: */*\r\n";
        hdr = hdr + "Accept-Language: en-US,en;q=0.5\r\n";
        hdr = hdr + "Cookie: " + &cookie + "\r\n";
        hdr = hdr + "\r\n";
        hdr
    }

    fn recv_data<T: Socketable> (&mut self, sock: &mut T)  {
        let mut data = Vec::new();
        let mut done = false;

        self.http_parser_.clear();

        while !done {
            let recv_size = sock.read_to_end_gen (&mut data).unwrap();
            if recv_size <= 0 {
                self.err_ = "connection closed.".to_string();
                return;
            }

            self.http_parser_.parse(&mut data);
            done = !self.http_parser_.is_partial();
            data.clear();
        }

        if !self.http_parser_.is_ok() && !self.http_parser_.is_redirect() {
            self.err_ = format!("HTTP Error (Response Code: {})", self.http_parser_.get_rep_code());
        }
    }

    fn request (&mut self, url: &mut Box<Url>) -> bool {
        self.redir_history.clear ();
        let mut done: bool = false;

        while !done && self.err_.is_empty() {
            if !self.check_redir(url) {
                break;
            }

            match self.dns_.get_sock_addr (&url.host_str().unwrap().to_string()) {
                Some(addr) => {
                    let ip = addr.ip().octets();
                    let port = match url.port() {
                        Some(p) => p,
                        None => match url.scheme() {
                            "https" => 443,
                            "http" => 80,
                            _ => 80,
                        },
                    };

                    let mut tcp_s: TcpStream = match TcpStream::connect (format!("{}.{}.{}.{}:{}",ip[0],ip[1],ip[2],ip[3],port))  {
                        Ok(s) => s,
                        _ => {
                            self.err_ = format!("\"{}.{}.{}.{}:{}\"",ip[0],ip[1],ip[2],ip[3],port).to_string();
                            continue;
                        },
                    };

                    let cook = self.cookie_.lock().unwrap().get_cookie(url);
                    let send_data = self.make_http_header (url, cook);

                    if url.scheme() == "https" {
                        let connector = SslConnectorBuilder::new(SslMethod::tls()).unwrap().build();
                        let mut tcp_ssl: SslStream<TcpStream> = connector.connect(&url.host_str().unwrap(), tcp_s).unwrap();
                        
                        match tcp_ssl.write(send_data.as_bytes())
                        {
                            Ok(_) => self.recv_data(&mut tcp_ssl),
                            Err(e) => { error! ("Tcp ssl write error {}", e)},
                        };
                    }
                    else {
                        match tcp_s.write(send_data.as_bytes())
                        {
                            Ok(_) => self.recv_data (&mut tcp_s),
                            Err(e) => { error! ("Tcp write error {}", e)},
                        };
                    }

                    self.cookie_.lock().unwrap().insert(&self.http_parser_.get_cookie(), url);
                },
                None => {},
            };

            if self.http_parser_.is_redirect() && !self.http_parser_.get_location().is_empty() {
                if let Ok(u) = Url::parse(&self.http_parser_.get_location()) {
                    *url =  Box::new(u);
                }
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
            
            let url_opt = self.url_q.lock().unwrap().get_next_url ();
            if url_opt == None {
                thread::sleep(Duration::from_secs(3));
                continue;
            }

            let mut url = url_opt.unwrap();

            if self.request (&mut url) {
                self.output_ = self.out_dir_.clone() + &self.thread_idx.to_string() + "_" + &html_cnt.to_string() + ".html";
                html_cnt = html_cnt + 1;
                let out_path = Path::new(&self.output_);
                let display = out_path.display();
                let mut out_file = match File::create(&out_path) {
                    Ok(f) => f,
                    Err(why) => panic!("couldn't open {}: {}", display, why.description()),
                };
                match out_file.write_all (self.http_parser_.get_body ().as_bytes()) {
                    _ => {;},
                }
                html_parser.parse (self.http_parser_.get_body().to_string());
                self.url_q.lock().unwrap().insert (&mut url, &mut html_parser.extract_link_url_list ());
            }
            else {
                error!("{} --> {}", url.as_str(), self.err_);
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