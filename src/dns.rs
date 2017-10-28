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

    fn check_lookup_host (&self, host: &String) -> bool
    {
        match net::lookup_host(&host) {
            Ok(_) => { return true; },
            Err(_) => {return false;},
        };
    }

    pub fn get_sock_addr (&mut self, host: &String) -> Option<SocketAddrV4> {
        let mut sock_v4 = SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 80);
        let mut e = sock_v4.clone();
        match self.cache_.get (host) {
            Some(a) => {
                e = a.clone();
            },
            None => {
                if self.check_lookup_host (host) == true
                {
                    for sock_v4_6 in net::lookup_host(&host).unwrap()
                    {
                        match sock_v4_6 {
                            SocketAddr::V4(x) => { e = x; break; },
                            SocketAddr::V6(_) => { e = sock_v4; } ,
                        };
                    }
                    sock_v4 = e.clone();
                }
            },
        };
        if sock_v4 == SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 80) {
            return None;
        }
        self.cache_.insert (host.clone(), e);
        Some(e)
    }
}

