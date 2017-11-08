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
                        if addrs.len() != 0
                        {
                            let addr = match addrs[0] {
                                SocketAddr::V4(x) => Some(x),
                                SocketAddr::V6(_) => None,
                            };
                            addr
                        }
                        else {
                            None
                        }
                    },
                    Err(msg) => {
                        error! ("lookup host error. {}", msg);
                        None
                    },
                }
            },
        };

        if addr_rlt != None
        {
            self.cache_.insert (host.clone(), addr_rlt.unwrap().clone());
        }
        addr_rlt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_localhost() {
        let mut dns = Dns::new();
        let rlt = dns.get_sock_addr(&"loocalhost".to_string());
        assert_eq!(None, rlt);
        println! ("Dns : {:?}", dns);
    }

    #[test]
    fn get_localhost2() {
        let mut dns = Dns::new();
        let rlt = dns.get_sock_addr(&"loocalhost".to_string());
        assert_eq!(None, rlt);
        let rlt2 = dns.get_sock_addr(&"loocalhost".to_string());
        assert_eq!(None, rlt2);
        println! ("Dns : {:?}", dns);
    }

    #[test]
    fn get_google() {
        let mut dns = Dns::new();
        let rlt = dns.get_sock_addr(&"www.google.com".to_string());
        assert_ne!(None, rlt);
        println! ("Dns : {:?}", dns);
    }

    #[test]
    fn get_google3() {
        let mut dns = Dns::new();
        let rlt = dns.get_sock_addr(&"www.google.com".to_string());
        assert_ne!(None, rlt);
        let rlt2 = dns.get_sock_addr(&"www.google.com".to_string());
        assert_ne!(None, rlt2);
        assert_eq!(rlt, rlt2);
        let rlt3 = dns.get_sock_addr(&"www.google.com".to_string());
        assert_ne!(None, rlt3);
        assert_eq!(rlt, rlt3);
        println! ("Dns : {:?}", dns);
    }

    #[test]
    fn get_google_yahoo() {
        let mut dns = Dns::new();
        let rlt = dns.get_sock_addr(&"www.google.com".to_string());
        assert_ne!(None, rlt);
        let rlt2 = dns.get_sock_addr(&"www.yahoo.com".to_string());
        assert_ne!(None, rlt2);
        assert_ne!(rlt, rlt2);
        println! ("Dns : {:?}", dns);
    }
}