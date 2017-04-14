use std::collections::HashMap;
use std::str;
//use url_parser::Url;


#[derive(Debug, Clone, PartialOrd,Ord,PartialEq,Eq)] enum State { INIT, TAG, COMMENT, }
#[derive(Debug, Clone, PartialOrd,Ord,PartialEq,Eq)] enum TagType { A, FRAME, SCRIPT, BODY, BODY_END, ERROR,}
#[derive(Debug, Clone, PartialOrd,Ord,PartialEq,Eq,Hash)] enum AttrType { HREF, SRC, UNKNOWN, }
#[derive(Debug, Clone, PartialOrd,Ord,PartialEq,Eq)] enum ParserType { NONE = 0, LINK_URL = 0x01, FRAME_SRC = 0x02, BODY_TEXT = 0x04, }
#[derive(Debug, Clone, PartialOrd,Ord,PartialEq,Eq)] enum ScriptState { S_INIT, S_SCRIPT, S_OUT1, S_OUT2 }

#[derive(Debug, Clone,PartialEq,Eq)]
pub struct HtmlParser {
    parser_type_: ParserType,
    state_: State,
    url_list_: Vec<String>,
    frame_: Vec<String>,
    plain_text_: String,
    is_body_: bool,
    latin_set_: HashMap<String, u8>,
}

impl HtmlParser {
    fn clear (&mut self) {
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

        p.latin_set_.insert ("quot".to_string(), 34); p.latin_set_.insert ("amp".to_string(), 38); p.latin_set_.insert ("lt".to_string(), 60); p.latin_set_.insert ("gt".to_string(), 62);
        p.latin_set_.insert ("nbsp".to_string(), 160); p.latin_set_.insert ("iexcl".to_string(), 161); p.latin_set_.insert ("cent".to_string(), 162); p.latin_set_.insert ("pound".to_string(), 163);
        p.latin_set_.insert ("curren".to_string(), 164); p.latin_set_.insert ("yen".to_string(), 165); p.latin_set_.insert ("brvbar".to_string(), 166); p.latin_set_.insert ("brkbar".to_string(), 166);
        p.latin_set_.insert ("sect".to_string(), 167); p.latin_set_.insert ("uml".to_string(), 168);  p.latin_set_.insert ("die".to_string(), 168); p.latin_set_.insert ("copy".to_string(), 169);
        p.latin_set_.insert ("ordf".to_string(), 170); p.latin_set_.insert ("laquo".to_string(), 171); p.latin_set_.insert ("not".to_string(), 172); p.latin_set_.insert ("shy".to_string(), 173);
        p.latin_set_.insert ("reg".to_string(), 174); p.latin_set_.insert ("macr".to_string(), 175); p.latin_set_.insert ("hibar".to_string(), 175); p.latin_set_.insert ("deg".to_string(), 176);
        p.latin_set_.insert ("plusmn".to_string(), 177); p.latin_set_.insert ("sup2".to_string(), 178); p.latin_set_.insert ("sup3".to_string(), 179); p.latin_set_.insert ("acute".to_string(), 180);
        p.latin_set_.insert ("micro".to_string(), 181); p.latin_set_.insert ("para".to_string(), 182); p.latin_set_.insert ("middot".to_string(), 183); p.latin_set_.insert ("cedil".to_string(), 184);
        p.latin_set_.insert ("sup1".to_string(), 185); p.latin_set_.insert ("ordm".to_string(), 186); p.latin_set_.insert ("raquo".to_string(), 187); p.latin_set_.insert ("frac14".to_string(), 188);
        p.latin_set_.insert ("frac12".to_string(), 189); p.latin_set_.insert ("frac34".to_string(), 190); p.latin_set_.insert ("iquest".to_string(), 191); p.latin_set_.insert ("Agrave".to_string(), 192);
        p.latin_set_.insert ("Aacute".to_string(), 193); p.latin_set_.insert ("Acirc".to_string(), 194); p.latin_set_.insert ("Atilde".to_string(), 195); p.latin_set_.insert ("Auml".to_string(), 196);
        p.latin_set_.insert ("Aring".to_string(), 197); p.latin_set_.insert ("AElig".to_string(), 198); p.latin_set_.insert ("Ccedil".to_string(), 199); p.latin_set_.insert ("Egrave".to_string(), 200);
        p.latin_set_.insert ("Eacute".to_string(), 201); p.latin_set_.insert ("Ecirc".to_string(), 202); p.latin_set_.insert ("Euml".to_string(), 203); p.latin_set_.insert ("Igrave".to_string(), 204);
        p.latin_set_.insert ("Iacute".to_string(), 205); p.latin_set_.insert ("Icird".to_string(), 206); p.latin_set_.insert ("Iuml".to_string(), 207); p.latin_set_.insert ("ETH".to_string(), 208);
        p.latin_set_.insert ("Ntilde".to_string(), 209); p.latin_set_.insert ("Ograve".to_string(), 210); p.latin_set_.insert ("Oacute".to_string(), 211); p.latin_set_.insert ("Ocirc".to_string(), 212);
        p.latin_set_.insert ("Otilde".to_string(), 213); p.latin_set_.insert ("Ouml".to_string(), 214); p.latin_set_.insert ("times".to_string(), 215); p.latin_set_.insert ("Oslash".to_string(), 216);
        p.latin_set_.insert ("Ugrave".to_string(), 217); p.latin_set_.insert ("Uacute".to_string(), 218); p.latin_set_.insert ("Ucirc".to_string(), 219); p.latin_set_.insert ("Uuml".to_string(), 220);
        p.latin_set_.insert ("Yacute".to_string(), 221); p.latin_set_.insert ("THORN".to_string(), 222); p.latin_set_.insert ("szlig".to_string(), 223); p.latin_set_.insert ("agrave".to_string(), 224);
        p.latin_set_.insert ("aacute".to_string(), 225); p.latin_set_.insert ("acirc".to_string(), 226); p.latin_set_.insert ("atilde".to_string(), 227); p.latin_set_.insert ("auml".to_string(), 228);
        p.latin_set_.insert ("aring".to_string(), 229); p.latin_set_.insert ("aelig".to_string(), 230); p.latin_set_.insert ("ccedil".to_string(), 231); p.latin_set_.insert ("egrave".to_string(), 232);
        p.latin_set_.insert ("eacute".to_string(), 233); p.latin_set_.insert ("ecirc".to_string(), 234); p.latin_set_.insert ("euml".to_string(), 235); p.latin_set_.insert ("igrave".to_string(), 236);
        p.latin_set_.insert ("iacute".to_string(), 237); p.latin_set_.insert ("icirc".to_string(), 238); p.latin_set_.insert ("iuml".to_string(), 239); p.latin_set_.insert ("eth".to_string(), 240);
        p.latin_set_.insert ("ntilde".to_string(), 241); p.latin_set_.insert ("ograve".to_string(), 242); p.latin_set_.insert ("oacute".to_string(), 243); p.latin_set_.insert ("ocirc".to_string(), 244);
        p.latin_set_.insert ("otilde".to_string(), 245); p.latin_set_.insert ("ouml".to_string(), 246); p.latin_set_.insert ("divide".to_string(), 247); p.latin_set_.insert ("oslash".to_string(), 248); 
        p.latin_set_.insert ("ugrave".to_string(), 249); p.latin_set_.insert ("uacute".to_string(), 250); p.latin_set_.insert ("ucirc".to_string(), 251); p.latin_set_.insert ("uuml".to_string(), 252);
        p.latin_set_.insert ("yacute".to_string(), 253); p.latin_set_.insert ("thorn".to_string(), 254); p.latin_set_.insert ("yuml".to_string(), 255);

        p
    }

    fn convert_latin_set (&self, data: &[u8], b: usize, e: usize, c: &mut char) -> usize {
        let mut cur = b;
        let mut set_e = cur + 1;

        while set_e < e && data[set_e] != ';' as u8 && !(data[set_e] as char).is_whitespace() {
            set_e = set_e + 1;
        }

        if set_e != e && !(data[set_e] as char).is_whitespace() {
            let code: u8;
            if data[cur] == '#' as u8 {
                cur = cur + 1;
                code = str::from_utf8(&data[cur..set_e]).unwrap().to_string().parse().unwrap();
            }
            else {
                code = *self.latin_set_.get(str::from_utf8(&data[cur..set_e]).unwrap()).unwrap();
            }

            if code > 0 {
                if code > 127 {
                    *c = ' ';
                }
                else {
                    *c = code as char;
                }
                return set_e;
            }
        }
        *c = data[b] as char;
        b
    }

    fn get_tag_type (&self, data: &[u8], prev: usize, b: usize) -> TagType {
        let tag = str::from_utf8(&data[prev..b]).unwrap().to_uppercase();

        let rlt = match tag.as_ref() {
            "A" => TagType::A,
            "FRAME" => TagType::FRAME,
            "SCRIPT" => TagType::SCRIPT,
            "BODY" => TagType::BODY,
            "/BODY" => TagType::BODY_END,
            &_ => TagType::ERROR
        };
        rlt
    }

    fn parse_attribute (&self, data: &[u8], mut b: usize, e: usize, attr_list: &mut HashMap<AttrType, String>) -> usize {
        while b < e {
            while (data[b] as char).is_whitespace() {
                b = b + 1;
            }

            let prev = b;

            while data[b] != '=' as u8 && !(data[b] as char).is_whitespace() {
                b = b + 1;
            }

            let attr = str::from_utf8(&data[prev..b]).unwrap().to_uppercase();
            let attr_type = match attr.as_ref() {
                "HREF" => AttrType::HREF,
                "SRC" => AttrType::SRC,
                _ => AttrType::UNKNOWN,
            };

            while b < e && data[b] == '=' as u8 || (data[b] as char).is_whitespace() {
                b = b + 1;
            }

            if b == e {
                break;
            }

            let value;
            let val_b;
            if data[b] == '"' as u8 || data[b] == '\'' as u8 {
                //let sep = data[b];
                val_b = b + 1;
                while b < e && (data[b] as char).is_whitespace() {
                    b = b + 1;
                }
                value = str::from_utf8(&data[val_b..b]).unwrap().to_string();
                b = b + 1;
            }
            else {
                val_b = b;
                while b < e && !(data[b] as char).is_whitespace() {
                    b = b + 1;
                }
                value = str::from_utf8(&data[val_b..b]).unwrap().to_string();
            }

            if attr_type != AttrType::UNKNOWN {
                attr_list.insert (attr_type, value);
            }
        }
        e
    }

    fn get_attr_value (&self, attr_list: &HashMap<AttrType, String>, attr_type: AttrType) -> String {
        let val = match attr_list.get(&attr_type) {
            Some(x) => x.clone(),
            None => String::new(),
        };

        val
    }

    fn remove_script (&self, data: &[u8], mut b: usize, e: usize) -> usize {
        let mut state: ScriptState = ScriptState::S_INIT;
        for _ in b..e {
            match state {
                ScriptState::S_INIT => {
                    if data[b] == '<' as u8 {
                        return b;
                    }
                    else if data[b] == '>' as u8 {
                        state = ScriptState::S_SCRIPT;
                    }
                },

                ScriptState::S_SCRIPT => {
                    if data[b] == '<' as u8 {
                        state = ScriptState::S_OUT1;
                    }
                },

                ScriptState::S_OUT1 => {
                    if data[b] == '/' as u8 {
                        state = ScriptState::S_OUT2;
                    }
                    else {
                        state = ScriptState::S_SCRIPT;
                    }
                },

                ScriptState::S_OUT2 => {
                    let prev = b;
                    while b < e && (data[b] as char).is_alphanumeric() {
                        b = b + 1;
                    }

                    if self.get_tag_type (data, prev, b) == TagType::SCRIPT {
                        if b < e && data[b] == '>' as u8 {
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

    fn tag (&mut self, data: &[u8], mut b: usize) -> usize {
        let prev = b;
        let e = data.len();

        while b < e && !(data[b] as char).is_whitespace() && data[b] != '>' as u8 && data[b] != '<' as u8 {
            b = b + 1;
        }

        if data[b] == '<' as u8 || b == e {
            return b;
        }

        let mut tag_e = b;
        while tag_e < e && data[tag_e] != '>' as u8 {
            tag_e = tag_e + 1;
        }

        let mut attr_list = HashMap::new();
        let val;

        match self.get_tag_type (data, prev, b) {
            TagType::A => {
                if self.parser_type_.clone() as u8 & ParserType::LINK_URL as u8 > 0 {
                    self.parse_attribute (data, b, tag_e, &mut attr_list);
                    val = self.get_attr_value (&attr_list, AttrType::HREF);
                    if !val.is_empty() {
                        self.url_list_.push (val);
                    }
                }
            },

            TagType::FRAME => {
                if self.parser_type_.clone() as u8 & ParserType::FRAME_SRC as u8 > 0 {
                    self.parse_attribute (data, b, tag_e, &mut attr_list);
                    val = self.get_attr_value (&attr_list, AttrType::SRC);
                    if !val.is_empty() {
                        self.frame_.push (val);
                    }
                }
            },

            TagType::SCRIPT => {
                tag_e = self.remove_script (data, b, e);
            },

            TagType::BODY => {
                if self.parser_type_.clone() as u8 & ParserType::BODY_TEXT as u8 > 0 {
                    self.is_body_ = true;
                }
            },

            TagType::BODY_END => {
                if self.is_body_.clone() {
                    self.is_body_ = false;
                }
            },

            TagType::ERROR => {}
        };
        self.state_ = State::INIT;
        tag_e
    }

    pub fn parse (&mut self, html: String) -> bool {
        self.clear();
        let data = html.as_bytes();
        let mut b: usize = 0;
        let e: usize = b + data.len();

        while b < e {
            match self.state_ {
                State::INIT => {
                    if data[b] == '<' as u8 {
                        self.state_ = State::TAG;
                    }
                    else if self.is_body_ {
                        let mut c: char = data[b].clone() as char;
                        if c == '&' {
                            b = self.convert_latin_set (data, b, e, &mut c);
                        }
                        let mut temp = self.plain_text_.clone();
                        if !c.is_whitespace() || self.plain_text_.is_empty() || !temp.pop().unwrap().is_whitespace() {
                            self.plain_text_.push(c);
                        }
                    }
                },

                State::TAG => {
                    if data[b] == '!' as u8 && b+2 < e && data[b+1] == '-' as u8 && data[b+2] == '>' as u8 {
                        self.state_ = State::COMMENT;
                        b = b + 2;
                    }
                    else {
                        b = self.tag (data, b);
                    }
                },

                State::COMMENT => {
                    if data[b] == '-' as u8 && b+2 < e && data[b+1] == '-' as u8 && data[b+2] == '>' as u8 {
                        self.state_ = State::INIT;
                        b = b + 2;
                    }
                },
            };
            b = b + 1;
        }
        true
    }

    pub fn extract_link_url_list (&self) ->Vec<String> {
        self.url_list_.clone()
    }

    pub fn extract_frame_url_list (&self) -> Vec<String> {
        self.frame_.clone()
    }
}