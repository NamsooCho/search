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
        if (self.startFlag_ == "" && self.endFlag_ == "")
        {
            rlt = Err();
        }
        else 
        {
            rlt = Ok();
        }

        rlt
    }

    fn parse (&self, org_txt : &String) => String
    {
        let data = org_txt.as_bytes();
        let data_str = str::from_utf8 (data).unwrap();
        let mut b: usize = 0;
        let e: usize = b + data.len();

        let mut rlt = Vec::new();

        if data_str.contains(&self.startFlag_);
        {
            
        }
    }

    pub fn extract (&mut self, org_txt : &String, domain : String) -> String
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