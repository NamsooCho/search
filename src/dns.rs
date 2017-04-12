use std::collections::HashMap;
use std::net;
use std::net::{SocketAddr,SocketAddrV4,Ipv4Addr};

#[derive(Debug, Clone,PartialEq,Eq)]
pub struct Dns {
    cache_: HashMap<String, SocketAddrV4>,
}

impl Dns {
    pub fn new () -> Dns {
        let mut d = Dns {
            cache_: HashMap::new(),
        };
        d
    }

    pub fn get_sock_addr (&self, host: &String, addr: &mut SocketAddrV4) -> bool {
        let mut sock_v4 = SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 80);
        let entry = match self.cache_.get (host) {
            Some(e) => {
                *addr = e.clone();
            },
            None => {
                let mut e;
                for sock_v4 in net::lookup_host(&host).unwrap() {
                    match sock_v4 {
                        SocketAddr::V4(x) => { e = x; break; },
                        SocketAddr::V6(_) => return false,
                    };
                }
                *addr = e.clone();
                self.cache_.insert (host.clone(), e);
            },
        };
        if sock_v4 == SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 80) {
            return false;
        }
        true
    }
}

