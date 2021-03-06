use std::collections::HashMap;
//use cookie::Cookie;

#[derive(Debug, Clone, PartialOrd,Ord,PartialEq,Eq)] enum State { INIT, HeaderPartial, BodyPartial, }
#[derive(Debug, Clone, PartialOrd,Ord,PartialEq,Eq)] enum ChunkState { ChunkInit, ChunkPartial, }
#[derive(Debug, Clone, PartialOrd,Ord,PartialEq,Eq)] enum Method { GET, POST, RESPONSE, ERROR, }

#[derive(Debug, Clone, PartialOrd,Ord,PartialEq,Eq)]
pub struct HttpParser {
    state_: State,
    buf_: String,
    method_: Method,
    body_: String,
    rep_code_: usize,
    is_chunk_: bool,
    contents_len_: usize,
    chunk_state_: ChunkState,
    chunk_size_: i32,
    host_: String,
    location_: String,
    cookie_: Vec<String>,
}

impl HttpParser {
    pub fn new () -> HttpParser {
        HttpParser {
            state_: State::INIT,
            buf_: String::new(),
            method_: Method::ERROR,
            body_:String::new(),
            rep_code_: 0,
            is_chunk_: false,
            contents_len_: 0,
            chunk_state_: ChunkState::ChunkInit,
            chunk_size_: 0,
            host_: String::new(),
            location_: String::new(),
            cookie_: Vec::new(),
        }
    }

    pub fn clear (&mut self) {
        self.state_ = State::INIT;
        self.chunk_state_ = ChunkState::ChunkInit;
        self.rep_code_ = 0;
        self.contents_len_ = 0;
        self.chunk_size_ = 0;
        self.method_ = Method::ERROR;

        self.host_.clear();
        self.location_.clear();
        self.body_.clear();
        self.buf_.clear();
        self.cookie_.clear();
        self.is_chunk_ = false;
    }

    pub fn get_body (&self) -> String {
        self.body_.clone()
    }

    pub fn get_cookie (&self) -> Vec<String> {
        self.cookie_.clone()
    }

    pub fn get_location (&self) -> String { self.location_.clone() }
    pub fn is_ok(&self) -> bool { return self.rep_code_ >= 200 && self.rep_code_ < 300; }
    pub fn is_redirect(&self) -> bool { return self.rep_code_ >= 300 && self.rep_code_ < 400; }
    pub fn is_partial(&self) -> bool { return self.state_ != State::INIT; }
    pub fn get_rep_code(&self) -> usize { self.rep_code_ }

    fn get_field_data (&self, data: &[u8], mut b: usize, e: usize, temp: &mut String) {
        //let data_ = data.clone();
        //let data = (*data_).as_bytes();
        let mut rlt = Vec::new();

        //b = b + data.len();
        while b < e && (data[b] == ':' as u8 || data[b] == ' ' as u8) {
            b = b + 1;
        }

        while b < e && data[b] != '\r' as u8 && data[b] != '\n' as u8 {
            rlt.push(data[b]);
            b = b + 1;
        }

        *temp = String::from_utf8(rlt).unwrap();
    }

    fn parse_field (&mut self, data: &[u8]) -> bool {
        let con_len = "content-length".to_string();
        let host = "host".to_string();
        let location = "location".to_string();
        let cookie = "set-cookie".to_string();
        let trans_enc = "transfer-encoding".to_string();
        let chunk = "chunked".to_string();

        let mut temp = String::new();
        let b: usize = 0;
        let e = data.len() as usize;

        if e == 0 {
            return true;
        }
        
        match (data[b] as char).to_uppercase().next().unwrap() {
            'C' => {
                if b + con_len.len() < e && con_len == String::from_utf8_lossy(&data[b..b+con_len.len()]).to_lowercase() {
                    self.get_field_data (data, b + con_len.len(), e, &mut temp);
                    self.contents_len_ = temp.parse().unwrap();
                }
            },

            'H' => {
                if b + host.len() < e && host == String::from_utf8_lossy(&data[b..b+host.len()]).to_lowercase() {
                    self.get_field_data (data, b+ host.len(), e, &mut self.host_.clone());
                }
            },

            'L' => {
                if b + location.len() < e && location == String::from_utf8_lossy(&data[b..b+location.len()]).to_lowercase() {
                    self.get_field_data (data, b+ location.len(), e, &mut self.location_.clone());
                }
            },

            'S' => {
                if b + cookie.len() < e && cookie == String::from_utf8_lossy(&data[b..b+cookie.len()]).to_lowercase() {
                    self.get_field_data (data, b+ cookie.len(), e, &mut temp);
                    self.cookie_.push(temp.clone());
                }
            },

            'T' => {
                if b + trans_enc.len() < e && trans_enc == String::from_utf8_lossy(&data[b..b+trans_enc.len()]).to_lowercase() {
                    self.get_field_data (data, b+ trans_enc.len(), e, &mut temp);
                    if chunk == temp {
                        self.is_chunk_ = true;
                    }
                }
            },

            '\r' => {
                return false;
            },

            _ => {return false;}
        }
        true
    }

    fn parse_header (&mut self, data: &[u8]) {
        let mut hdr_partial = true;
        let b = 0;
        let e = data.len();

        let mut prev = b;
        let mut cur = b;

        while hdr_partial && cur != e {
            if cur != b && data[cur - 1] == '\r' as u8 && data[cur] == '\n' as u8 {
                if !self.buf_.is_empty() {
                    self.buf_.push_str (&String::from_utf8_lossy(&data[prev..cur]));
                    let temp = self.buf_.clone();
                    if self.method_ == Method::ERROR {
                        self.method_ = self.parse_method (&temp.as_bytes());
                    }
                    else {
                        hdr_partial = self.parse_field (&temp.as_bytes());
                    }
                    self.buf_.clear();
                }
                else {
                    if self.method_ == Method::ERROR {
                        self.method_ = self.parse_method (&data[prev.. cur -1]);
                    }
                    else {
                        hdr_partial = self.parse_field (&data[prev.. cur -1]);
                    }
                }
                prev = cur + 1;
            }
            cur = cur + 1;
        }
        if hdr_partial {
            self.state_ = State::HeaderPartial;
            self.buf_.push_str (&String::from_utf8_lossy(&data[prev.. e]));
        }
        else {
            self.state_ = State::BodyPartial;
            if self.contents_len_ > 0 {
                self.body_.reserve (self.contents_len_ as usize);
            }
            self.parse_body (&data[prev..e]);
        }
    }

    fn parse_method (&mut self, data: &[u8]) -> Method {
        let mut method_list = HashMap::new();

        method_list.insert ("GET".to_string(), Method::GET);
        method_list.insert ("POST".to_string(), Method::POST);
        method_list.insert ("HTTP/1.1".to_string(), Method::RESPONSE);

        let data_str = String::from_utf8_lossy(&data);
        let mut split_iter = data_str.split_whitespace();
        let method: String = match split_iter.next() {
            Some(x) => x.to_uppercase(),
            None => "".to_string()
        };

        let code: Method = match method_list.get(&method) {
            Some(x) => x.clone(),
            None => Method::ERROR,
        };

        if code == Method::RESPONSE {
            self.rep_code_ = match split_iter.next() {
                Some(x) => x.to_string().parse().unwrap(),
                None => 404
            };
        }
        return code;
    }

    fn get_chunk_size (&mut self, data: &[u8]) -> usize {
        let mut cur: usize = 0;
        let mut prev: usize = 0;
        for i in 0..data.len() as usize {
            if !(data[i] as char).is_digit(16) {
                continue;
            }
            prev = i;

            if (data[i] as char).is_digit(16) {
                continue;
            }

            self.chunk_size_ = String::from_utf8_lossy(&data[prev..i]).parse().unwrap();

            if data[i-1] == ('\r' as u8) && data[i] == ('\n' as u8) {
                cur = i;
                break;
            }
        }

        if cur >= data.len() as usize {
            self.chunk_size_ = -1;
            cur = prev;
        }
        else {
            cur = cur + 1;
        }
        
        cur
    }
    
    fn append_body (&mut self, data: &[u8], mut b:  usize) -> usize {
        if self.chunk_size_ < 0 {
            self.buf_.push_str (&String::from_utf8_lossy(&data[b..]));
            return data.len() as usize;
        }

        let data_size = data.len() as usize - b;

        if data_size as i32 > self.chunk_size_ {
            self.body_.push_str (&String::from_utf8_lossy(&data[b..b+self.chunk_size_ as usize]));
            b = b + self.chunk_size_ as usize;
            self.chunk_state_ = ChunkState::ChunkInit;
        }
        else {
            self.body_.push_str (&String::from_utf8_lossy(&data[b..]));
            self.chunk_state_ = ChunkState::ChunkPartial;
            self.chunk_size_ = self.chunk_size_ - data_size as i32;
            b = data.len() as usize;
        }

        b
    }

    fn parse_chunk (&mut self, data: &[u8]) {
        let mut b = 0;
        let e = b + data.len() as usize;

        while b < e {
            if self.chunk_state_ == ChunkState::ChunkInit {
                b = self.get_chunk_size (data);
                if self.chunk_size_ == -1 {
                    self.buf_.push_str (&String::from_utf8_lossy(&data[b..e]));
                    self.chunk_state_ = ChunkState::ChunkPartial;
                    break;
                }
                else if self.chunk_size_ == 0 {
                    self.state_ == State::INIT;
                    break;
                }
                else {
                    b = self.append_body (data, b);
                }
            }
            else if self.chunk_state_ == ChunkState::ChunkPartial {
                if e - b < 2 && self.chunk_size_ < 0 {
                    self.buf_.push_str (&String::from_utf8_lossy(&data[b..e]));
                    b = b + 1;
                }
                else {
                    if self.buf_.len() as usize != 0 {
                        let prev = b;
                        while b < e && data[b] != ('\r' as u8) && data[b+1] != ('\n' as u8) {
                            b = b + 1;
                        }
                        if b < e {
                            b = b + 2;
                            self.buf_.push_str (&String::from_utf8_lossy(&data[prev..b]));
                            let temp = self.buf_.clone();
                            self.get_chunk_size (temp.as_bytes());
                            self.buf_.clear();
                            if self.chunk_size_ > 0 {
                                b = self.append_body(data, b);
                            }
                            else if self.chunk_size_ == 0 {
                                self.chunk_state_ = ChunkState::ChunkInit;
                                break;
                            }
                            else {
                                panic!("ERROR");
                            }
                        }
                        else {
                            self.buf_.push_str(&String::from_utf8_lossy(&data[prev..b]));
                        }
                    }
                    else {
                        if self.chunk_size_ < 0 {
                            b = self.get_chunk_size (data);
                        }
                        b = self.append_body (data, b);
                    }
                }
            }
        }
    }

    fn parse_body (&mut self, data: &[u8]) {
        if self.is_chunk_ {
            self.parse_chunk (data);
        }
        else {
            self.body_.push_str(&String::from_utf8_lossy(&data));
            if self.body_.len() as usize >= self.contents_len_ {
                self.state_ = State::INIT;
            }
        }
    }
    
    pub fn parse (&mut self, data: &mut[u8]) {
        match self.state_ {
            State::INIT   => { self.clear (); self.parse_header (&data); },
            State::HeaderPartial   => self.parse_header (&data),
            State::BodyPartial   => self.parse_body (&data),
        };
    }
}