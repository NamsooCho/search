

enum State { INIT, TAG, COMMENT };
enum TagType { A, FRAME, SCRIPT, BODY, BODY_END };
enum AttrType { HREF, SRC };
enum ParserType { NONE = 0, LINK_URL = 0x01, FRAME_SRC = 0x02, BODY_TEXT = 0x04, ALL = LINK_URL|FRAME_SRC|BODY_TEXT };

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

    fn tag (&self, data: &[u8], b: i32) -> i32 {
        let prev = b;
        let e = data.len();

        while b < e && !data[b].is_whitespace() && data[b] != '>' && data[b] != '<' {
            b = b + 1;
        }

        if data[b] == '<' || b == e
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