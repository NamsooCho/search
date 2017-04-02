

enum State { INIT, TAG, COMMENT };
enum TagType { A, FRAME, SCRIPT, BODY, BODY_END };
enum AttrType { HREF, SRC };
enum ParserType { NONE = 0, LINK_URL = 0x01, FRAME_SRC = 0x02, BODY_TEXT = 0x04, ALL = LINK_URL|FRAME_SRC|BODY_TEXT };

pub struct HtmlParser {
    parser_type_: ParserType,
    state_: State,
    url_list: vec![String],
    frame: vec![String],
    plain_text: String,
    is_body: bool,
}

impl HtmlParser {
    
}