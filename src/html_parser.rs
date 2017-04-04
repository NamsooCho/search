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
    latin_set_: HashMap,
}

impl HtmlParser {
    fn clear (&self) {
        self.state_ = State::INIT;
        self.url_list_.clear();
        self.frame_.clear();
        self.plain_text_.clear();
        self.is_body_ = false;
        self.latin_set_.clear();
    }

    pub fn new() -> HtmlParser {
        let mut p = HtmlParser {
            parser_type_: ParserType::NONE,
            state_: State::INIT,
            url_list_: Vec::new(),
            frame_: Vec::new(),
            plain_text_: String::new(),
            is_body_: false,
            latin_set_: HashMap::new(),
        };

        p.latin_set_.insert ("quot", 34); p.latin_set_.insert ("amp", 38); p.latin_set_.insert ("lt", 60); p.latin_set_.insert ("gt", 62);
        p.latin_set_.insert ("nbsp", 160); p.latin_set_.insert ("iexcl", 161); p.latin_set_.insert ("cent", 162); p.latin_set_.insert ("pound", 163);
        p.latin_set_.insert ("curren", 164); p.latin_set_.insert ("yen", 165); p.latin_set_.insert ("brvbar", 166); p.latin_set_.insert ("brkbar", 166);
        p.latin_set_.insert ("sect", 167); p.latin_set_.insert ("uml", 168);  p.latin_set_.insert ("die", 168); p.latin_set_.insert ("copy", 169);
        p.latin_set_.insert ("ordf", 170); p.latin_set_.insert ("laquo", 171); p.latin_set_.insert ("not", 172); p.latin_set_.insert ("shy", 173);
        p.latin_set_.insert ("reg", 174); p.latin_set_.insert ("macr", 175); p.latin_set_.insert ("hibar", 175); p.latin_set_.insert ("deg", 176);
        p.latin_set_.insert ("plusmn", 177); p.latin_set_.insert ("sup2", 178); p.latin_set_.insert ("sup3", 179); p.latin_set_.insert ("acute", 180);
        p.latin_set_.insert ("micro", 181); p.latin_set_.insert ("para", 182); p.latin_set_.insert ("middot", 183); p.latin_set_.insert ("cedil", 184);
        p.latin_set_.insert ("sup1", 185); p.latin_set_.insert ("ordm", 186); p.latin_set_.insert ("raquo", 187); p.latin_set_.insert ("frac14", 188);
        p.latin_set_.insert ("frac12", 189); p.latin_set_.insert ("frac34", 190); p.latin_set_.insert ("iquest", 191); p.latin_set_.insert ("Agrave", 192);
        p.latin_set_.insert ("Aacute", 193); p.latin_set_.insert ("Acirc", 194); p.latin_set_.insert ("Atilde", 195); p.latin_set_.insert ("Auml", 196);
        p.latin_set_.insert ("Aring", 197); p.latin_set_.insert ("AElig", 198); p.latin_set_.insert ("Ccedil", 199); p.latin_set_.insert ("Egrave", 200);
        p.latin_set_.insert ("Eacute", 201); p.latin_set_.insert ("Ecirc", 202); p.latin_set_.insert ("Euml", 203); p.latin_set_.insert ("Igrave", 204);
        p.latin_set_.insert ("Iacute", 205); p.latin_set_.insert ("Icird", 206); p.latin_set_.insert ("Iuml", 207); p.latin_set_.insert ("ETH", 208);
        p.latin_set_.insert ("Ntilde", 209); p.latin_set_.insert ("Ograve", 210); p.latin_set_.insert ("Oacute", 211); p.latin_set_.insert ("Ocirc", 212);
        p.latin_set_.insert ("Otilde", 213); p.latin_set_.insert ("Ouml", 214); p.latin_set_.insert ("times", 215); p.latin_set_.insert ("Oslash", 216);
        p.latin_set_.insert ("Ugrave", 217); p.latin_set_.insert ("Uacute", 218); p.latin_set_.insert ("Ucirc", 219); p.latin_set_.insert ("Uuml", 220);
        p.latin_set_.insert ("Yacute", 221); p.latin_set_.insert ("THORN", 222); p.latin_set_.insert ("szlig", 223); p.latin_set_.insert ("agrave", 224);
        p.latin_set_.insert ("aacute", 225); p.latin_set_.insert ("acirc", 226); p.latin_set_.insert ("atilde", 227); p.latin_set_.insert ("auml", 228);
        p.latin_set_.insert ("aring", 229); p.latin_set_.insert ("aelig", 230); p.latin_set_.insert ("ccedil", 231); p.latin_set_.insert ("egrave", 232);
        p.latin_set_.insert ("eacute", 233); p.latin_set_.insert ("ecirc", 234); p.latin_set_.insert ("euml", 235); p.latin_set_.insert ("igrave", 236);
        p.latin_set_.insert ("iacute", 237); p.latin_set_.insert ("icirc", 238); p.latin_set_.insert ("iuml", 239); p.latin_set_.insert ("eth", 240);
        p.latin_set_.insert ("ntilde", 241); p.latin_set_.insert ("ograve", 242); p.latin_set_.insert ("oacute", 243); p.latin_set_.insert ("ocirc", 244);
        p.latin_set_.insert ("otilde", 245); p.latin_set_.insert ("ouml", 246); p.latin_set_.insert ("divide", 247); p.latin_set_.insert ("oslash", 248); 
        p.latin_set_.insert ("ugrave", 249); p.latin_set_.insert ("uacute", 250); p.latin_set_.insert ("ucirc", 251); p.latin_set_.insert ("uuml", 252);
        p.latin_set_.insert ("yacute", 253); p.latin_set_.insert ("thorn", 254); p.latin_set_.insert ("yuml", 255);

        p
    }
    
    fn convert_latin_set (&self, b: &i32, e: &i32, c: &char) -> i32 {
        let mut cur = b;
        let mut set_e = cur + 1;

        while set_e < e && data[set_e] != ';' && !data[set_e].is_whitespace() {
            set_e = set_e + 1;
        }

        if set_e != e && !data[set_e].is_whitespace() {
            let mut code = 0;
            if data[cur] == '#' {
                cur = cur + 1;
                code = String::from_utf8_lossy(data[cur..set_e]).parse();
            }
            else {
                code = self.latin_set_.get(String::from_utf8_lossy(data[cur..set_e])).unwrap();
            }

            if code > 0 {
                if code > 127 {
                    c = ' ';
                }
                else {
                    c = code;
                }
                return set_e;
            }
        }
        c = data[b];
        b
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
        let mut state: ScriptState = ScriptState::S_INIT;
        for c in b..e {
            match state {
                ScriptState::S_INIT => {
                    if data[b] == '<' {
                        return b;
                    }
                    else if data[b] == '>' {
                        state = ScriptState::S_SCRIPT;
                    }
                },

                ScriptState::S_SCRIPT => {
                    if data[b] == '<' {
                        state = ScriptState::S_OUT1;
                    }
                },

                ScriptState::S_OUT1 => {
                    if data[b] == '/' {
                        state = ScriptState::S_OUT2;
                    }
                    else {
                        state = ScriptState::S_SCRIPT;
                    }
                },

                ScriptState::S_OUT2 => {
                    let mut prev = b;
                    while b < e && data[b].is_alphanumeric() {
                        b = b + 1;
                    }

                    if self.get_tag_type (data, prev, b) == TagType::SCRIPT {
                        if b < e && data[b] == '>' {
                            return b;
                        }
                    }
                    else {
                        state = ScriptState::S_SCRIPT;
                        b = prev;
                    }
                }
            }
        }
        b
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