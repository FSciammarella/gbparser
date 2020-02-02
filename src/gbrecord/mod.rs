use std::collections::HashMap;
use std::collections::HashSet;

use pest::iterators::Pair;

use crate::gbparser::*;
use crate::alphabet::Alphabet;
use crate::conformation::Conformation;
use crate::references::Reference;
use crate::features::Feature;
use crate::gbreader::ParseOpts;

mod rev_comp;
mod extract_region;
mod fields;
pub mod format_feature;

#[derive(Debug,Clone)]
pub struct GBRecord{
    pub accession: Option<String>,
    pub version: Option<usize>,
    pub length: Option<usize>,
    pub alphabet: Option<Alphabet>,
    pub conformation:Option<Conformation>,
    pub section:Option<String>,
    pub date:Option<String>,
    pub definition: Option<String>,
    pub dblink: Option<HashMap<String,String>>,
    pub keywords: Option<HashSet<String>>,
    pub source: Option<String>,
    pub organism: Option<String>,
    pub taxonomy: Option<HashSet<String>>,
    pub references: Option<Vec<Reference>>,
    pub comment: Option<String>,
    pub features: Option<Vec<Feature>>,
    pub sequence: Option<String>,
    pub(crate) raw_data: Option<String>,
}
impl GBRecord {
    fn new() -> GBRecord {
        GBRecord {
            accession: None,
            version: None,
            length: None,
            conformation: None,
            alphabet: None,
            section: None,
            date: None,
            definition: None,
            dblink: None,
            keywords: None,
            source: None,
            organism: None,
            taxonomy: None,
            references: None,
            comment: None,
            features: None,
            sequence: None,
            raw_data:None,
        }
    }
    pub fn from(record: Pair<Rule>, options:&HashSet<ParseOpts>) -> GBRecord {
        let mut new_record = GBRecord::new();
        if options.contains(&ParseOpts::HoldBuffer){
            new_record.raw_data=Some(record.as_str().to_string());
        }
        for section in record.into_inner() {
            match section.as_rule() {
                Rule::header => {
                    GBRecord::fill_header(section, &mut new_record, options)
                }
                Rule::features => {
                    GBRecord::fill_features(section, &mut new_record, options)
                }
                Rule::origin => {
                    GBRecord::fill_origin(section, &mut new_record, options)
                }
                _ => unreachable!(),
            }
        }
        new_record
    }
}