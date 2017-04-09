use std::collections::HashMap;
use std::io::net::addrinfo;
use std::io::net::ip::IpAddr;

struct Dns {
    cache_: HashMap<String, String>,
}

impl Dns {
    pub fn new () -> Dns {
        let mut d = Dns {
            cache_: HashMap::new();
        };
        d
    }

    pub fn get_sock_addr (&self, host: &String, addr: &String) -> bool {
        let entry = metch self.cache_.get (host) {
            Some(e) => e,
            None => String::new("");
        };

        let ip: = Vec<IpAddr>::new();
        if !entry.is_empty() {
            ip = addrinfo::get_host_addresses (&host).unwrap();
            if ip.len() > 0 {
                let address = match ip[0] {
                    Ipv4Addr(a,b,c,d) => format!("{}.{}.{}.{}", a,b,c,d),
                    _ => String::new(),
                };
                *addr = address.clone();
                self.cache_.insert (host, address);
            }
            else {
                return false;
            }
        }
        else {
            *addr = address;
        }
        true
    }
}

