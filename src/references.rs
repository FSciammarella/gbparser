use std::collections::HashSet;

use pest::iterators::Pair;

use crate::gbparser::*;
use crate::range::Range;

#[derive(Debug,Clone)]
pub struct Reference{
    pub range: Range,
    pub title: Option<String>,
    pub authors: Option<Vec<String>>,
    pub journal: Option<String>,
    pub pubmed: Option<String>,
    pub consortium: Option<String>,
    pub remark: Option<String>,
}
impl Reference{
    fn new()->Reference{
        Reference{
            range:Range::Single(0),
            title: None,
            authors: None,
            journal:None,
            pubmed:None,
            consortium:None,
            remark:None,
        }
    }
    pub fn from(ref_token:Pair<Rule>)->Reference{
        let mut new_ref = Self::new();
        for component in ref_token.into_inner(){
            match component.as_rule(){
                Rule::aut_name=>{
                    new_ref.authors = if let Some(mut h) =new_ref.authors.take(){
                        h.push(component.as_str().trim().to_string());
                        Some(h)
                    }else{
                        let mut auth_set = Vec::new();
                        auth_set.push(component.as_str().trim().to_string());
                        Some(auth_set)
                    }
                }
                Rule::pubmed=>{
                    new_ref.pubmed=Some(component.as_str().to_string());
                }
                Rule::consortium=>{
                    new_ref.consortium=Some(component.as_str().trim().to_string());
                }
                Rule::range=>{
                    new_ref.range=Range::from(component)
                }
                Rule::title=>{
                    let mut last_space=false;
                    let mut titl = String::new();
                    for ch in component.as_str().chars(){
                        match ch {
                            ' '|'\n'=>{
                                if !last_space{
                                    titl.push(' ');
                                }
                                last_space=true;
                            }
                            _=>{
                                titl.push(ch);
                                last_space=false
                            },
                        }
                    }
                    while let Some(c) = titl.pop(){
                        if c != ' ' && c != '\n' && c != '\t'{
                            titl.push(c);
                            break;
                        }
                    }
                    new_ref.title=Some(titl)
                }
                Rule::journal=>{
                    let mut last_space=false;
                    let mut jrnl = String::new();
                    for ch in component.as_str().chars(){
                        match ch {
                            ' '|'\n'=>{
                                if !last_space{
                                    jrnl.push(' ');
                                }
                                last_space=true;
                            }
                            _=>{
                                jrnl.push(ch);
                                last_space=false
                            },
                        }
                    }
                    while let Some(c) = jrnl.pop(){
                        if c != ' ' && c != '\n' && c != '\t'{
                            jrnl.push(c);
                            break;
                        }
                    }
                    new_ref.journal=Some(jrnl)
                }
                Rule::remark=>{
                    let mut last_space=false;
                    let mut rmrk = String::new();
                    for ch in component.as_str().chars(){
                        match ch {
                            ' '|'\n'=>{
                                if !last_space{
                                    rmrk.push(' ');
                                }
                                last_space=true;
                            }
                            _=>{
                                rmrk.push(ch);
                                last_space=false
                            },
                        }
                    }
                    while let Some(c) = rmrk.pop(){
                        if c != ' ' && c != '\n' && c != '\t'{
                            rmrk.push(c);
                            break;
                        }
                    }
                    new_ref.remark=Some(rmrk)
                }
                _=>()
            }
        }
        new_ref
    }
}