use std::collections::HashMap;
use std::net;
use std::net::{SocketAddr,SocketAddrV4};

#[derive(Debug, Clone,PartialEq,Eq)]
pub struct Dns {
    cache_: HashMap<String, SocketAddrV4>,
}

impl Dns {
    pub fn new () -> Dns {
        Dns {
            cache_: HashMap::new(),
        }
    }

    pub fn get_sock_addr (&mut self, host: &String) -> Option<SocketAddrV4> {
        let addr_rlt = match self.cache_.get(host) {
            Some(addr) => {
                return Some(*addr);
            },
            None => {
                match net::lookup_host (&host) {
                    Ok(a) => {
                        let addrs: Vec<SocketAddr> = a.collect();
                        let addr = match addrs[0] {
                            SocketAddr::V4(x) => Some(x),
                            SocketAddr::V6(_) => None,
                        };
                        addr
                    },
                    Err(msg) => {
                        error! ("lookup host error. {}", msg);
                        None
                    },
                }
            },
        };

        self.cache_.insert (host.clone(), addr_rlt.unwrap().clone());
        addr_rlt
    }
}

