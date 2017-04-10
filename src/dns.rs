use std::collections::HashMap;
use std::io::net::addrinfo;
use std::io::net::ip::IpAddr;
use std::net;
use std::net::SocketAddrV4;

pub struct Dns {
    cache_: HashMap<String, String>,
}

impl Dns {
    pub fn new () -> Dns {
        let mut d = Dns {
            cache_: HashMap::new(),
        };
        d
    }

    pub fn get_sock_addr (&self, host: &String, addr: &mut SocketAddrV4) -> bool {
        let entry = match self.cache_.get (host) {
            Some(e) => {
                *addr = entry.clone();
            },
            None => {
                let sock_v4 = match net::lookup_host (&host).unwrap().next() {
                    Some(x) => {
                        match x {
                            V4(x) => x,
                        }
                    },
                    _ => return false,
                };
                *addr = sock_v4.clone();
                self.cache_.insert (host, sock_v4);
            },
        };

        true
    }
}

