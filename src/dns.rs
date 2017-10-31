use std::collections::HashMap;
use std::net;
use std::net::{SocketAddr,SocketAddrV4,Ipv4Addr};

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
        let sock_v4 = match self.cache_.get(host) {
            Some(addr) => addr,
            None => {
                let addrs = match net::lookup_host (&host) {
                    Ok(a) => a.filter (|s| s.is_ipv4()).collect(),
                    Err(msg) => { error! ("lookup host error. {}", msg); vec![] },
                };

                if addrs.len() == 0
                {
                    SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 80)
                }
                else 
                {
                    match addrs[0] 
                    {
                        SocketAddr::V4(x) => x,
                        SocketAddr::V6(_) => SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 80),
                    };
                }
            },
        };
        
        if sock_v4 == SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 80) {
            return None;
        }
        self.cache_.insert (host.clone(), sock_v4.clone());
        Some(sock_v4)
    }
}

