// crate domain_dic
mod domain_dic
use domain_dic::domainDic

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

    fn getTags (&mut self, domain : String) -> Result<()>
    {

    }

    pub fn extract (&mut self, org_txt : String, domain : String) -> String
    {
        let mut rlt_str : String = '';

        getTags (domain);
    }
}