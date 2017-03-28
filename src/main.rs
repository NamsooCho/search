extern crate getopts;
extern crate multimap;
use getopts::Options;
use std::env;
use std::thread;
#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use] 
extern crate enum_primitive;
extern crate num;

mod http_socket_thread;
mod sync_q;
mod url_parser;
mod cookie;
mod cookie_container;
mod http_parser;

use http_socket_thread::HttpSocketThread;
use cookie_container::CookieContainer;

struct Args {
    q_limit: u32,
    seed: String,
    out_dir: String,
    timeout: u32,
    sock_cnt: u32,
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    println!("{}", opts.usage(&brief));
}

fn main() {
    //let mut sockArr: Vec<HttpSocketThread> = Vec::new();
    let mut i = 0;

    env_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("q", "", "set queue limit", "NUMBER");
    opts.optopt("s", "", "set seed url", "URL");
    opts.optopt("o", "", "output directory", "DIR");
    opts.optopt("t", "", "set timeout", "NUMBER");
    opts.optopt("c", "", "set socket count", "NUMBER");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) =>  m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let q_limit: u32 = match matches.opt_str("q") {
        Some(x) => x.parse().unwrap(),
        None => 100,
    };

    let seed = match matches.opt_str("s") {
        Some(x) => "http://".to_string() + &x,
        None => "http://www.naver.com".to_string(),
    };

    let out_dir = match matches.opt_str("o") {
        Some(x) => x + "/result.txt",
        None => "./result.txt".to_string(),
    };

    let timeout: u32 = match matches.opt_str("t") {
        Some(x) => x.parse().unwrap(),
        None => 1000,
    };

    let sock_cnt: u32 = match matches.opt_str("c") {
        Some(x) => x.parse().unwrap(),
        None => 8,
    };

    let arg: Args = Args {
        q_limit: q_limit,
        seed: seed,
        out_dir: out_dir,
        timeout: timeout,
        sock_cnt: sock_cnt,
        };

    info!("crawling start...");
    let mut children = vec![];
    let mut sock_arr = vec![];
    let mut cookie = CookieContainer::new();
    for i in 0..arg.sock_cnt {
        let mut sock = HttpSocketThread::new(&mut cookie);
        sock_arr.push(sock.clone());
        children.push(thread::spawn(move || {
            sock.initiate();
        }));
    }

    for child in children {
        let _ = child.join();
    }
}
