use std::collections::HashMap;

pub struct HttpParser {
    state_: State,
    buf_: String,
    method_: String,
    body_: String,
    rep_code: i32,
    is_chunk_: bool,
    contents_len_: i32,
    chunk_state_: ChunkState,
}

enum State { INIT, HEADER_PARTIAL, BODY_PARTIAL, }
enum ChunkState { CHUNK_INIT, CHUNK_PARTIAL, }
enum Method { GET, POST, RESPONSE, ERROR, }

impl HttpParser {
    fn clear_parser (&self) {
        
    }

    fn parse_header (&self, data: &String, size: i16) {
        let mut hdr_partial = true;
        let mut b = &data.as_ptr();
        let mut e = b + size;

        let mut prev = b;
        let mut cur = b;

        for i in 0..size {
            if prev != b && *(cur - 1) == '\r' && *cur == '\n' {
                if !self.buf_.is_empty() {
                    self.buf_.push_str (&data[prev..cur]);
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
                    if self.method_ == 0 {
                        self.method_ = parse_method (&data[prev.. cur -1], cur - prev - 1);
                    }
                    else {
                        hdr_partial = parse_field (&data[prev.. cur -1], cur - prev - 1);
                    }
                }
                prev = cur + 1;
            }
            cur = cur + 1;
        }
        if hdr_partial {
            self.state_ = State::HEADER_PARTIAL;
            self.buf_.push_str (&data[prev.. e]);
        }
        else {
            self.state_ = State::BODY_PARTIAL;
            if self.contents_len_ > 0 {
                self.body_.reserve (self.contents_len_);
            }
            self.parse_body (&data[prev..e], e - prev);
        }
    }

    fn parse_method (&self, data: &String, size: i16) Method {
        let mut method_list = HashMap::new();

        method_list.insert ("GET", Method::GET);
        method_list.insert ("POST", Method::POST);
        method_list.insert ("HTTP", Method::RESPONSE);

        let mut b = &data.as_ptr();
        let mut e = b + size;
        let mut temp = b;

        let mut iter = data.split_whitespace();
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

    fn get_chunk_size (&self, data: &String, size: i16, len: &i16) {
        ß
    }
    fn parse_chunk (&self, data: &String, size: i16) {
        let mut b = data.as_ptr();
        let mut e = b + size;

        while b < e {
            if self.chunk_state_ == ChunkState::CHUNK_INIT {ß
                b= self.
            }
        }
    }

    fn parse_body (&self, data: &String, size: i16) {
        if self.is_chunk_ {
            self.parse_chunk (data, size);
        }
        else {
            self.body_.push_str(data);
            if self.body_.len() >= self.contents_len_ {
                self.state_ = State::INIT;
            }
        }
    }
    
    pub fn parse (&self, data: String, size: i16) {
        match self.state_ {
            0   => self.clear_parser (),
            1   => self.parse_header (&data, size),
            2   => self.parse_body (&data, size),
        };
    }
}