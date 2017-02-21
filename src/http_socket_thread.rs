use std::path::Path;

struct DNS {

}

struct HttpSocketThread {
    continue_: bool,
    url_q: SyncQ,
    output_: Path,
    redir_history: Vec<String>,
}

impl DNS {
    fn get_sock_addr (&str host, &str )
}

impl HttpSocketThread {
    fn initiate () {
        self.continue_ = true;
        self.redir_history = Vec<String>::new();
        thread_function ();
    }

    fn thread_function () {
        let html_parser: HtmlParse = HtmlParser::new();

        while (continue_) {
            if (url_q.full ())
                break;
            
            let url: Url = url_q.get_next_url ();
            if (request (&url)) {
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

    fn request (url: &Url) => bool {
        if (url.url.is_empty())
            return false;

        redir_history.clear ();
        let err_: String = "".to_string();
        let done: bool = false;
        let ret: bool = false;

        while (!done && err_.is_empty()) {
            if (!check_redir(url))
                break;

            let addr: IpAddr = match url.to_socket_addrs().next() {
                Some(x) => x.ip(),
                None => IpAddr::V4(Ipv4Addr::new(0.0.0.0)),
            }
        }
    }
}