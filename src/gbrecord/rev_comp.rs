use crate::gbrecord::GBRecord;

impl GBRecord{
    pub(crate) fn reverse_comp(subject:&str)->String{
        let mut result=String::new();
        for ch in subject.chars().rev(){
            result.push(match ch.to_ascii_lowercase(){
                'a'=>'t',
                't'|'u'=>'a',
                'g'=>'c',
                'c'=>'g',
                'y'=>'r',
                'r'=>'y',
                's'=>'s',
                'w'=>'w',
                'k'=>'m',
                'b'=>'v',
                'd'=>'h',
                'h'=>'d',
                'v'=>'b',
                'n'=>'n',
                '-'=>'-',
                _=>unreachable!()
            });
        }
        result
    }
}