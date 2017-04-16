#![feature(lookup_host)]
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
extern crate time;

mod http_socket_thread;
mod sync_q;
mod url_parser;
mod cookie;
mod http_parser;
mod html_parser;
mod dns;

use http_socket_thread::HttpSocketThread;
use cookie::Cookie;
use std::sync::{Arc,Mutex};
use sync_q::SyncQ;

struct Args {
    q_limit_: u32,
    seed_: String,
    out_dir_: String,
    timeout_: u32,
    sock_cnt_: u32,
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    println!("{}", opts.usage(&brief));
}

fn main() {
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
        q_limit_: q_limit,
        seed_: seed,
        out_dir_: out_dir,
        timeout_: timeout,
        sock_cnt_: sock_cnt,
        };

    info!("crawling start...");
    let mut children = vec![];
    let mut sock_arr = vec![];
    let cookie_ = Arc::new(Mutex::new(Cookie::new()));
    let queue_ = Arc::new(Mutex::new(SyncQ::new(&arg.seed_, q_limit)));
    for _ in 0..arg.sock_cnt_ {
        let cookie_c = cookie_.clone();
        let queue_c = queue_.clone();
        let mut cookie = cookie_c.lock().unwrap();
        let mut queue = queue_c.lock().unwrap();
        let mut sock = HttpSocketThread::new(&mut queue, &mut cookie);
        sock_arr.push(sock.clone());
        let cookie_ = cookie_.clone();
        let queue_ = queue_.clone();
        children.push(thread::spawn(move || {
            let mut cookie = cookie_.lock().unwrap();
            let mut queue = queue_.lock().unwrap();
            sock.initiate(&mut queue, &mut cookie);
        }));
    }

    for child in children {
        let _ = child.join();
    }
}
