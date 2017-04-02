use std::collections::HashMap;
use cookie::Cookie;

pub struct HttpParser {
    state_: State,
    buf_: String,
    method_: String,
    body_: String,
    rep_code_: i32,
    is_chunk_: bool,
    contents_len_: i32,
    chunk_state_: ChunkState,
    chunk_size_: i32,
    host_: String,
    location_: String,
    cookie_: Cookie,
}

enum State { INIT, HEADER_PARTIAL, BODY_PARTIAL, }
enum ChunkState { CHUNK_INIT, CHUNK_PARTIAL, }
enum Method { GET, POST, RESPONSE, ERROR, }

impl HttpParser {
    fn clear_parser (&self) {
        self.state_ = State::INIT;
        self.chunk_state_ = ChunkState::CHUNK_INIT;
        self.rep_code_ = self.contents_len_ = self.chunk_size_ = 0;
        self.method_.clear();

        self.host_.clear();
        self.location_.clear();
        self.body_.clear();
        self.buf_.clear();
        self.cookie_.clear();
        self.is_chunk_ = false;
    }

    fn parse_header (&self, data: &[u8]) {
        let mut hdr_partial = true;
        let mut b = 0;
        let mut e = data.len();

        let mut prev = b;
        let mut cur = b;

        for i in 0..e {
            if prev != b && data[cur - 1] == '\r' && data[cur] == '\n' {
                if !self.buf_.is_empty() {
                    self.buf_.push_str &(String::from_utf8_lossy(data[prev..cur]));
                    let temp = self.buf_.clone();
                    if self.method_ == 0 {
                        self.method_ = self.parse_method (temp, temp.len());
                    }
                    else {
                        hdr_partial = parse_field (temp, temp.len());
                    }
                    self.buf_.clear();
                }
                else {
                    if self.method_len() == 0 {
                        self.method_ = parse_method (data[prev.. cur -1], cur - prev - 1);
                    }
                    else {
                        hdr_partial = parse_field (data[prev.. cur -1], cur - prev - 1);
                    }
                }
                prev = cur + 1;
            }
            cur = cur + 1;
        }
        if hdr_partial {
            self.state_ = State::HEADER_PARTIAL;
            self.buf_.push_str &(String::from_utf8_lossy(data[prev.. e]));
        }
        else {
            self.state_ = State::BODY_PARTIAL;
            if self.contents_len_ > 0 {
                self.body_.reserve (self.contents_len_);
            }
            self.parse_body (data[prev..e], e - prev);
        }
    }

    fn parse_method (&self, data: &[u8]) Method {
        let mut method_list = HashMap::new();

        method_list.insert ("GET", Method::GET);
        method_list.insert ("POST", Method::POST);
        method_list.insert ("HTTP", Method::RESPONSE);

        let mut iter = String::from_utf8_lossy(data).split_whitespace();
        let mut method: String = iter.next().unwrap().to_uppercase();
        let code = match method_list.get(&method) {
            Some(x) => x,
            None => Method::ERROR,
        };

        if code == Method::RESPONSE {
            self.rep_code_ = iter.next().unwrap().parse().unwrap();
        }
        return code;
    }

    fn get_chunk_size (&self, data: &[u8]) -> i32 {
        let mut cur = 0;
        for i in 0..data.len() {
            if !data[i].is_digit(16) {
                continue;
            }
            let prev = i;

            if data[i].is_digit(16) {
                continue;
            }

            let self.chunk_size_ = String::from_utf8_lossy(data[prev..i]).parse().unwrap();

            if data[i-1] == '\r' && data[i] == '\n' {
                cur = i;
                break;
            }
        }

        if cur >= data.len() {
            self.chunk_size_ = -1;
            cur = prev;
        }
        else {
            cur = cur + 1;
        }
        
        cur
    }
    
    fn append_body (&self, data: &[u8], b: mut i32) -> i32 {
        if self.chunk_size_ < 0 {
            self.buf_.add (data[b..]);
            return data.len();
        }

        let data_size = data.len() - b;

        if data_size > self.chunk_size_ {
            self.body_.add (data[b..b+self.chunk_size_]);
            b = b + self.chunk_size_;
            self.chunk_state_ = ChunkState::CHUNK_INIT;
        }
        else {
            self.body_.add (data[b..]);
            self.chunk_state = ChunkState::CHUNK_PARTIAL;
            self.chunk_size_ = self.chunk_size_ - data_size;
            b = data.len();
        }

        b
    }

    fn parse_chunk (&self, data: &[u8]) {
        let mut b = 0;
        let mut e = b + data.len();

        while b < e {
            if self.chunk_state_ == ChunkState::CHUNK_INIT {
                b = self.get_chunk_size (data);
                if self.chunk_size_ == -1 {
                    self.buf_.add (&String::from_utf8_lossy(data[b..e]));
                    self.chunk_state_ = ChunkState::CHUNK_PARTIAL;
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
            else if self.chunk_state_ == ChunkState::CHUNK_PARTIAL {
                if e - b < 2 && self.chunk_size_ < 0 {
                    self.buf_.add (&String::from_utf8_lossy(data[b..e]));
                    b = b + 1;
                }
                else {
                    if self.buf_.len() != 0 {
                        let mut prev = b;
                        while b < e && data[b] != '\r' && data[b+1] != '\n' {
                            b = b + 1;
                        }
                        if b < e {
                            b = b + 2;
                            self.buf_.add (&String::from_utf8_lossy(data[prev..b]));
                            let temp = self.buf_;
                            self.get_chunk_size (temp.as_bytes());
                            self.buf_.clear();
                            if self.chunk_size_ > 0 {
                                b = self.append_body(data, b);
                            }
                            else if self.chunk_size_ == 0 {
                                self.chunk_state_ = State::INIT;
                                break;
                            }
                            else {
                                panic!("ERROR");
                            }
                        }
                        else {
                            self.buf_.add(&String::from_utf8_lossy(data[prev..b]));
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

    fn parse_body (&self, data: &[u8]) {
        if self.is_chunk_ {
            self.parse_chunk (data);
        }
        else {
            self.body_.push_str(&String::from_utf8_lossy(data));
            if self.body_.len() >= self.contents_len_ {
                self.state_ = State::INIT;
            }
        }
    }
    
    pub fn parse (&self, data_: String) {
        let mut data = data_.as_bytes();
        match self.state_ {
            0   => self.clear_parser (),
            1   => self.parse_header (&data),
            2   => self.parse_body (&data),
        };
    }
}