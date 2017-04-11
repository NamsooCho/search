use std::collections::HashMap;
use std::net;
use std::net::SocketAddrV4;

#[derive(Debug, Clone, PartialOrd,Ord,PartialEq,Eq)]
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
        let mut sock_v4 = SocketAddrV4::new();
        let entry = match self.cache_.get (host) {
            Some(e) => {
                *addr = e.clone();
            },
            None => {
                for sock_v4 in net::lookup_host (&host)? {
                    match sock_v4 {
                        Some(x) => {
                            match x {
                            SocketAddrV4::V4(x) => x,
                            };
                        },
                        _ => return false,
                    };
                    *addr = sock_v4.clone();
                    self.cache_.insert (host, sock_v4);
                    break;
                };
            },
        };

        true
    }
}

