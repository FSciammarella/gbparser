use crate::gbparser::*;
use pest::iterators::Pair;

#[derive(Debug,Clone)]
pub enum Range{
    BaseRange(usize,usize),
    Single(usize),
    Complement(Box<Range>),
    Join(Vec<Box<Range>>)
}

impl Range{
    pub fn from(range_token:Pair<Rule>)->Range{
        for elem in range_token.into_inner(){
            match elem.as_rule() {
                Rule::single=>{
                    return Range::Single(elem.as_str().parse().unwrap())
                }
                Rule::base_range=>{
                    let mut insides = elem.into_inner();
                    let start = insides.next().unwrap().as_str().parse().unwrap();
                    let end = insides.next().unwrap().as_str().parse().unwrap();
                    return Range::BaseRange(start,end)
                }
                Rule::complement=>{
                    let mut insides = elem.into_inner();
                    return Range::Complement(Box::new(Range::from(insides.next().unwrap())))
                }
                Rule::join=>{
                    let mut res = Vec::new();
                    let insides = elem.into_inner();
                    for range in insides{
                        res.push(Box::new(Range::from(range)))
                    }
                    return Range::Join(res)
                }
                _=>unreachable!()
            }
        }
        unreachable!()
    }
}
