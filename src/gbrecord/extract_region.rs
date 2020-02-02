use crate::gbrecord::GBRecord;
use crate::range::Range;

impl GBRecord{
    pub fn extract_region(&self, range:&Range)->String{
        match range{
            Range::Single(p)=>{
                let mut result = String::new();
                result.push_str(self.sequence.as_ref().unwrap().get((*p-1)..=(*p-1)).unwrap());
                return result;
            }
            Range::BaseRange(start,end)=>{
                let mut result = String::new();
                result.push_str(self.sequence.as_ref().unwrap().get((start-1)..=(end-1)).unwrap());
                return result;
            }
            Range::Complement(r)=>{
                return Self::reverse_comp(&self.extract_region(r.as_ref()));
            }
            Range::Join(v)=>{
                let mut result =String::new();
                for rg in v{
                    result+=&self.extract_region(rg.as_ref());
                }
                return result;
            }
            _=>String::new(),
        }
    }
}