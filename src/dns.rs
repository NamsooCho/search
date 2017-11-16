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
        match self.cache_.get(host) {       // if domain exists in cache, returns with the value
            Some(a) => {
                return Some(*a);
            },
            None => { ; },
        };
        // must separate logic into two blocks because self.cache_ is barrowed mutably
        match net::lookup_host (&host) {
            Ok(a) => {
                for addr in a {
                    match addr {
                        SocketAddr::V4(x) => { 
                            self.cache_.insert (host.clone(), x.clone());
                            return Some(x);
                        },
                        SocketAddr::V6(_) => { continue; },
                    };
                }
            },
            Err(msg) => {
                error! ("lookup host error. {}", msg);
            },
        };
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

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
    fn get_jysoft() { // if you test using big size site like 'www.google.com', beware that big site has multiple ip address.
        let mut dns = Dns::new();
        let rlt = dns.get_sock_addr(&"www.jyoungsoft.com".to_string());
        assert_eq!(&Ipv4Addr::new(210,180,0,153), rlt.unwrap().ip());
        println! ("Dns : {:?}", dns);
    }

    #[test]
    fn get_jysoft3() {
        let mut dns = Dns::new();
        let rlt = dns.get_sock_addr(&"www.jyoungsoft.com".to_string());
        assert_eq!(&Ipv4Addr::new(210,180,0,153), rlt.unwrap().ip());
        let rlt2 = dns.get_sock_addr(&"www.jyoungsoft.com".to_string());
        assert_eq!(&Ipv4Addr::new(210,180,0,153), rlt2.unwrap().ip());
        let rlt3 = dns.get_sock_addr(&"www.jyoungsoft.com".to_string());
        assert_eq!(&Ipv4Addr::new(210,180,0,153), rlt3.unwrap().ip());
        println! ("Dns : {:?}", dns);
    }

    #[test]
    fn get_jysoft_okky() {
        let mut dns = Dns::new();
        let rlt = dns.get_sock_addr(&"www.jyoungsoft.com".to_string());
        assert_eq!(&Ipv4Addr::new(210,180,0,153), rlt.unwrap().ip());
        let rlt2 = dns.get_sock_addr(&"www.okky.kr".to_string());
        assert_eq!(&Ipv4Addr::new(111,92,191,51), rlt2.unwrap().ip());
        println! ("Dns : {:?}", dns);
    }
}