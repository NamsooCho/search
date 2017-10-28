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
        let mut sock_v4 = SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 80);
        let mut e = sock_v4.clone();
        match self.cache_.get (host) {
            Some(a) => {
                e = a.clone();
            },
            None => {
                let lookups = net::lookup_host (&host);
                let mut addrs = vec![];
                match lookups {
                    Ok(a) => {
                        addrs = a.filter (|s| s.is_ipv4()).collect(); 
                    },
                    Err(_) => { print! ("lookup dns error.");}
                };

                for sock_v4_6 in addrs.into_iter()
                {
                    match sock_v4_6 {
                        SocketAddr::V4(x) => { e = x; break; },
                        SocketAddr::V6(_) => { e = sock_v4; } ,
                    };
                }
                sock_v4 = e.clone();
            },
        };
        if sock_v4 == SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 80) {
            return None;
        }
        self.cache_.insert (host.clone(), e);
        Some(e)
    }
}

