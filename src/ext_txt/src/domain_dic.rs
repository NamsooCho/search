use std::collections::HashMap;

pub struct StartEndPair
{
    start_flag_ : String,
    end_flag_ : String,
}

pub struct domainDic
{
    cutter_ : HashMap<String, StartEndPair>,
}

impl domainDic
{
    pub fn new() -> domainDic
    {
        let mut d = domainDic {
            cutter_ : HashMap::new(),
        }

        d
    }

    pub fn buildDic(&self) -> domainDic
    {
        
    }

    pub fn scanDic(&self, domain : String) -> StartEndPair
    {
        let search_rlt;
        if self.cutter_.contains_key(&domain)
        {
            search_rlt = self.cutter_[&domain];
        }
        else 
        {
            search_rlt = StartEndPair {"", ""};
        }

        let pair = StartEndPair {
            start_flag_ : search_rlt.start_flag_,
            end_flag_ : search_rlt.end_flag_,
        }

        pair
    }
}