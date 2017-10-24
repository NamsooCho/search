use std::collections::HashMap;

struct StartEndPair
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
}