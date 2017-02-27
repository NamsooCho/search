use std::path::Path;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream};

use sync_q::SyncQ;
use url_parser::Url;

struct DNS {

}

struct HttpSocketThread {
    continue_: bool,
    url_q: SyncQ,
    output_: Path,
    redir_history: Vec<String>,
}

impl DNS {
    fn get_sock_addr (host: &str, port: &str) {}
}

impl HttpSocketThread {
    fn make_http_header (url: &str, host: &str, cookie: &str) -> String {
        let mut hdr: String = "GET ";
        if (!url.is_empty()) {
            hdr += url;
        }
        else {
            hdr += "/";
        }
        hdr += " HTTP/1.1\r\n";
        hdr += "Host: " + host + "\r\n";
        hdr += "User-Agent: TinyCrawler\r\n";
        hdr += "Accept: */*\r\n";
        hdr += "Accept-Language: ko\r\n";
        hdr += "Cookie: " + cookie + "\r\n";
        hdr += "\r\n";
        hdr
    }

    fn request (&self, url: &Url) -> bool {
        if (url.url.is_empty()) {
            return false;
        }

        self.redir_history.clear ();
        let err_: String = "".to_string();
        let done: bool = false;
        let ret: bool = false;

        while (!done && err_.is_empty()) {
            //if (!check_redir(url)) {
            //    break;
            //}

            let addr: SocketAddr = match url.to_socket_addrs().next() {
                Some(x) => x,
                None => SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 80),
            };
            let mut tcp_s = TcpStream::connect (addr).unwrap();
            let send_data = self.make_http_header ("a", "b", "c");
        }
    }

    fn thread_function (&mut self) {
        //let html_parser: HtmlParse = HtmlParser::new();

        while (self.continue_) {
            if (self.url_q.full ()) {
                break;
            }
            
            let url: Url = self.url_q.get_next_url ();
            if (self.request (&url)) {
                self.output_ = self.output_.join(html_cnt.to_string()) + ".html".to_string();
                html_cnt = html_cnt + 1;
                let out_path = Path::new(self.output_);
                let display = out_path.display();
                let mut out_file = match File::create(&out_path) {
                    Ok(f) => f,
                    Err(why) => panic!("couldn't open {}: {}", display, why.description()),
                };
                out_file.write_all (self.get_body ().as_bytes()).unwrap();
                html_parser.parse (self.get_body().to_string(), self.get_body().to_string().len());
                self.url_q.insert (&url, html_parser.extract_link_url_list ());
            }
            else {
                error!("{} --> {}", url.url, self.get_err_msg());
            }
        }
    }

    fn initiate (&self) {
        self.continue_ = true;
        self.redir_history = Vec::new();
        thread_function ();
    }
}