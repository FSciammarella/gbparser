use std::collections::HashMap;

use pest::iterators::Pair;

use crate::gbparser::*;
use crate::range::Range;

#[derive(Debug,Clone)]
pub struct Feature{
    pub key: String,
    pub range: Range,
    pub properties: HashMap<String,String>,
}
impl Feature{
    fn new()->Feature{
        Feature{
            key:String::new(),
            range:Range::Single(0),
            properties:HashMap::new(),
        }
    }
    pub fn from(feature_token:Pair<Rule>)->Feature{
        let mut feat=Feature::new();
        //        eprintln!("{:#?}",feature_token);
        for field in feature_token.into_inner(){
            match field.as_rule() {
                Rule::range=>{
                    feat.range = Range::from(field);
                }
                Rule::feature_key=>{
                    feat.key = field.as_str().trim().to_string();
                }
                Rule::feature_property=>{
                    let mut innards = field.into_inner();
                    let property_name= innards.next().unwrap().as_str().trim().to_string();
                    let mut property_value=String::new();
                    let mut p_value =innards.next().unwrap().into_inner().next().unwrap();
                    match p_value.as_rule(){
                        Rule::string=>{
                            p_value=p_value.into_inner().next().unwrap();
                            if property_name.contains("translation"){
                                for c in p_value.as_str().chars(){
                                    match c {
                                        ' '|'\n'|'\t'=>(),
                                        _=> property_value.push(c),
                                    }
                                }
                            }else{
                                for c in p_value.as_str().chars(){
                                    match c {
                                        '\n'|'\t'=>property_value.push(' '),
                                        _=> property_value.push(c),
                                    }
                                }
                            }
                        }
                        _=>property_value=p_value.as_str().trim().to_string(),
                    }
                    feat.properties.insert(property_name,property_value);
                }
                _=>()
            }
        }
        feat
    }
}