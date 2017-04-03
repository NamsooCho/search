use std::collections::HashMap;


enum State { INIT, TAG, COMMENT };
enum TagType { A, FRAME, SCRIPT, BODY, BODY_END };
enum AttrType { HREF, SRC, UNKNOWN };
enum ParserType { NONE = 0, LINK_URL = 0x01, FRAME_SRC = 0x02, BODY_TEXT = 0x04, ALL = LINK_URL|FRAME_SRC|BODY_TEXT };
enum ScriptState { S_INIT, S_SCRIPT, S_OUT1, S_OUT2 };

pub struct HtmlParser {
    parser_type_: ParserType,
    state_: State,
    url_list_: vec![String],
    frame_: vec![String],
    plain_text_: String,
    is_body_: bool,
}

impl HtmlParser {
    fn clear (&self) {

    }

    fn convert_latin_set (&self, b: &i32, e: &i32, c: &char) -> i32 {

    }

    fn get_tag_type (&self, data: &[u8], prev: i32, b: i32) -> TagType {
        let tag = String::from_utf8_lossy(data[prev..b]).to_uppercase();

        let rlt = match tag {
            "A" => TagType::A,
            "FRAME" => TagType::FRAME,
            "SCRIPT" => TagType::SCRIPT,
            "BODY" => TagType::BODY,
            "/BODY" => TagType::BODY_END,
        }
        rlt
    }

    fn parse_attribute (&self, data: &[u8], b: i32, tag_e: i32, attr_list: &HashMap) -> i32 {
        while b < e {
            while data[b].is_whitespace() {
                b = b + 1;
            }

            let mut prev = b;

            while data[b] != '=' && !data[b].is_whitespace() {
                b = b + 1;
            }

            let attr = String::from_utf8_lossy(data[prev, b]).to_uppercase();
            let attr_type = match attr {
                "HREF" => AttrType::HREF,
                "SRC" => AttrType::SRC,
                _ => AttrType::UNKNOWN,
            }

            while b < e && (data[b] == '=' || data[b].is_whitespace() {
                b = b + 1;
            }

            if b == e {
                break;
            }

            let mut value = String::new();
            let mut val_b = 0;
            if data[b] == '"' || data[b] == '\' {
                let sep = data[b];
                val_b = b + 1;
                while b < e && data[b].is_whitespace() {
                    b = b + 1;
                }
                value = String::from_utf8_lossy(data[val_b..b]);
                b = b + 1;
            }
            else {
                val_b = b;
                while b < e && !data[b].is_whitespace() {
                    b = b + 1;
                }
                value = String::from_utf8_lossy(data[val_b..b]);
            }

            if attr_type != AttrType::UNKNOWN {
                attr_list.insert (value, attr_type);
            }
        }
        e
    }

    fn get_attr_value (&self, attr_list: &HashMap, attr_type: AttrType) -> String {
        let val = match attr_list.get(&attr_type) {
            Some(x) => x,
            None => "",
        }

        val
    }

    fn remove_script (&self, data: &[u8], b: i32, e: i32) -> i32 {

    }

    fn tag (&self, data: &[u8], b: i32) -> i32 {
        let prev = b;
        let e = data.len();

        while b < e && !data[b].is_whitespace() && data[b] != '>' && data[b] != '<' {
            b = b + 1;
        }

        if data[b] == '<' || b == e {
            return b;
        }

        let mut tag_e = b;
        while tag_e < e && data[tag_e] != '>' {
            tag_e = tag_e + 1;
        }

        let mut attr_list = HashMap::new();
        let mut val = String::new();

        match get_tag_type (data, prev, b) {
            TagType::A => {
                if self.parser_type_ as u8 & ParserType::LINK_URL as u8 {
                    b = self.parse_attribute (data, b, tag_e, &attr_list);
                    val = self.get_attr_value (&attr_list, AttrType::HREF);
                    if !val.is_empty() {
                        self.url_list_.push_str (&val);
                    }
                }
            },

            TagType::FRAME => {
                if self.parser_type_ as u8 & ParserType::FRAME_SRC as u8 {
                    b = self.parse_attribute (b, tag_e, &attr_list);
                    val = self.get_attr_value (&attr_list, AttrType::SRC);
                    if !val.is_empty() {
                        self.frame_.push_str (&val);
                    }
                }
            },

            TagType::SCRIPT => {
                tag_e = self.remove_script (b, e);
            },

            TagType::BODY => {
                if self.parser_type_ as u8 & ParserType::BODY_TEXT as u8 {
                    self.is_body_ = true;
                }
            },

            TagType::BODY_END => {
                if self.is_body_ {
                    self.is_body_ = false;
                }
            },
        }
        self.state_ = State::INIT;
        tag_e
    }

    pub fn parse (&self, html: String) -> bool {
        self.clear();
        let mut data = html.as_bytes();
        let mut b = 0;
        let e = b + data.len();

        while b < e {
            match self.state_ {
                State::INIT => {
                    if data[b] == '<' {
                        self.state_ = State::TAG;
                    }
                    else if self.is_body_ {
                        let c: char = data[b].clone();
                        if c == '&' {
                            b = self.convert_latin_set (&b, &e, &c);
                        }
                        let mut temp = self.plain_text_.clone();
                        if !c.is_whitespace() || self.plain_text_.is_empty() || !temp.pop().unwrap().is_whitespace() {
                            self.plain_text_ = self.plain_text_ + c;
                        }
                    }
                },

                State::TAG => {
                    if data[b] == '!' && b+2 < e && data[b+1] == '-' && data[b+2] == '>' {
                        self.state_ = State::COMMENT;
                        b = b + 2;
                    }
                    else {
                        b = self.tag (data, &b);
                    }
                },

                State::COMMENT => {
                    if data[b] == '-' && b+2 < e && data[b+1] == '-' && data[b+2] == '>' {
                        self.state_ = State::INIT;
                        b = b + 2;
                    }
                },
            };
            b = b + 1;
        }
    }
}