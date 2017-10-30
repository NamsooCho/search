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
        let sock_v4 : SocketAddrV4;
        let e : SocketAddrV4;

        match self.cache_.get (host) {
            Some(a) => {
                e = a.clone();
                sock_v4 = *a;
            },
            None => {
                let addrs : Vec<SocketAddr>;
                match net::lookup_host (&host) {
                    Ok(a) => 
                    {
                        addrs = a.filter (|s| s.is_ipv4()).collect();
                        if addrs.len() != 0
                        {
                            match addrs[0] 
                            {
                                SocketAddr::V4(x) => { e = x;},
                                SocketAddr::V6(_) => { 
                                    e = SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 80);
                                },
                            };
                        }
                        else 
                        {
                            e = SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 80);
                        }
                    },
                    Err(msg) => { e = SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 80); 
                                error! ("lookup host error. {}", msg);},
                };
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

