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

    pub fn get_sock_addr (&mut self, host: &String, addr: &mut SocketAddrV4) -> bool {
        let mut sock_v4 = SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 80);
        match self.cache_.get (host) {
            Some(e) => {
                *addr = e.clone();
                return true;
            },
            None => {
                let mut e = sock_v4.clone();
                for sock_v4_6 in net::lookup_host(&host).unwrap() {
                    match sock_v4_6 {
                        SocketAddr::V4(x) => { e = x; break; },
                        SocketAddr::V6(_) => { *addr = sock_v4; return false; } ,
                    };
                }
                *addr = e.clone();
                sock_v4 = e;
            },
        };
        if sock_v4 == SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 80) {
            *addr = sock_v4;
            return false;
        }
        self.cache_.insert (host.clone(), addr.clone());
        true
    }
}

