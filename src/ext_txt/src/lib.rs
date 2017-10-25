// crate domain_dic

use std::result::Result;
use std::str;

mod domain_dic;
use domain_dic::domainDic;

// build dictionary, search dictionary, extract html tag from html text string
// get domain from caller

pub struct html_extractor
{
    dic_ : domainDic,
    startFlag_ : String,
    endFlag_ : String,
}

impl html_extractor
{
    pub fn new () -> html_extractor
    {
        let mut h = html_extractor
        {
            dic_ : domainDic::new(),
            startFlag_ : "",
            endFlag_ : "",
        }

        h
    }

    fn getFlags (&mut self, domain : String) -> Result<_,_>
    {
        let pair = self.dic_.scanDic (domain);
        self.startFlag_ = pair.startFlag_;
        self.endFlag_ = pair.endFlag_;
        let rlt;
        if (&self.startFlag_ == "" && &self.endFlag_ == "")
        {
            rlt = Err();
        }
        else 
        {
            rlt = Ok();
        }

        rlt
    }

    fn del_tags (&self, data : &[u8], s : usize, e_tmp : usize) -> String
    {
        let rlt = Vec<u8>::new();
        let mut b: usize = s + self.startFlag_.len();
        let e: usize = e_tmp - b - self.endFlag_.len();

        while (b++ < e)
        {
            if data[b] == '<' as u8
            {
                while (b++ < e)
                {
                    if data[b] == '>' as u8
                    {
                        b++;
                        break;
                    }
                }
            }
            rlt.push (data[b]);
        }

        let rlt_str = str::from_utf8(rlt).unwrap().to_string();
        rlt_str
    }

    fn parse (&self, org_txt : String) -> String
    {
        let data = org_txt.as_bytes();
        let data_str = &org_txt;

        let mut rlt = "".to_string();

        if data_str.contains(&self.startFlag_) && data_str.contains(&self.endFlag_)
        {
            let s = data_str.find(&self.startFlag_).unwrap();
            let e = data_str.find(&self.endFlag_).unwrap();
            rlt = self.del_tags(data, s, e);
        }
        else
        {
            rlt = org_txt;
        }

        rlt
    }

    pub fn extract (&mut self, org_txt : String, domain : String) -> String
    {
        let mut rlt_str : String = '';

        match self.getFlags (domain)
        {
            Ok => {rlt_str = self.parse (org_txt);},
            Err => {return ""},
        };

        rlt_str
    }
}