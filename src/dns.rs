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
        let sock_v4 = match self.cache_.get(host) {
            Some(addr) => Some(*addr),
            None => {
                let addrs : Vec<SocketAddr> = match net::lookup_host (&host) {
                    Ok(a) => a.filter (|s| s.is_ipv4()).collect(),
                    Err(msg) => { error! ("lookup host error. {}", msg); vec![] },
                };

                if addrs.len() == 0
                {
                    None
                }
                else 
                {
                    match addrs[0] 
                    {
                        SocketAddr::V4(x) => Some(x),
                        SocketAddr::V6(_) => None,
                    }
                }
            },
        };
        
        if sock_v4 != None
        {
            self.cache_.insert (host.clone(), sock_v4.clone().unwrap());
        }
        sock_v4
    }
}

