use pest::iterators::Pair;

use crate::gbparser::*;
use crate::alphabet::Alphabet;
use crate::conformation::Conformation;
use crate::features::Feature;
use crate::references::Reference;
use crate::gbrecord::GBRecord;
use std::collections::{HashSet, HashMap};
use crate::gbreader::ParseOpts;
use crate::gbreader::ParseOpts::Sequence;

impl GBRecord{
    pub(crate) fn fill_header(header:Pair<Rule>,record: &mut GBRecord, options:&HashSet<ParseOpts>){
        for field in header.into_inner(){
            match field.as_rule() {
                Rule::accession=>{
                    if options.contains(&ParseOpts::Full)
                    || options.contains(&ParseOpts::Accession)
                    || options.contains(&ParseOpts::FullHeader){
                        record.accession = Some(String::from(field.as_str()))
                    }
                }
                Rule::size=>{
                    if options.contains(&ParseOpts::Full)
                        || options.contains(&ParseOpts::Size)
                        || options.contains(&ParseOpts::FullHeader){
                        record.length=Some(field.as_str().split(" ").collect::<Vec<&str>>()[0].parse::<usize>().unwrap())
                    }
                }
                Rule::alphabet=>{
                    if options.contains(&ParseOpts::Full)
                        || options.contains(&ParseOpts::Alphabet)
                        || options.contains(&ParseOpts::FullHeader) {
                        record.alphabet = Some(match field.into_inner().next().unwrap().as_rule() {
                            Rule::alphabet_dna => Alphabet::DNA,
                            Rule::alphabet_rna => Alphabet::RNA,
                            Rule::alphabet_protein => Alphabet::Protein,
                            _ => unreachable!()
                        })
                    }
                }
                Rule::conformation=>{
                    if options.contains(&ParseOpts::Full)
                        || options.contains(&ParseOpts::Conformation)
                        || options.contains(&ParseOpts::FullHeader) {
                        record.conformation= Some(match field.into_inner().next().unwrap().as_rule(){
                            Rule::linear =>  Conformation::Linear,
                            Rule::circular=> Conformation::Circular,
                            _=>unreachable!()
                        })
                    }
                }
                Rule::section=> {
                    if options.contains(&ParseOpts::Full)
                        || options.contains(&ParseOpts::Section)
                        || options.contains(&ParseOpts::FullHeader) {
                        record.section = Some(field.as_str().to_string())
                    }
                }
                Rule::date=> {
                    if options.contains(&ParseOpts::Full)
                        || options.contains(&ParseOpts::Date)
                        || options.contains(&ParseOpts::FullHeader) {
                        record.date = Some(field.as_str().to_string())
                    }
                }
                Rule::ver_accession=> {
                    if options.contains(&ParseOpts::Full)
                        || options.contains(&ParseOpts::Version)
                        || options.contains(&ParseOpts::FullHeader) {
                        let mut innards =field.into_inner();
                        innards.next().unwrap();
                        let version = innards.next().unwrap();
                        record.version = Some(version.as_str().parse::<usize>().unwrap())
                    }
                }
                Rule::definition=>{
                    if options.contains(&ParseOpts::Full)
                        || options.contains(&ParseOpts::Definition)
                        || options.contains(&ParseOpts::FullHeader) {
                        let mut last_space=false;
                        let mut definition = String::new();
                        for ch in field.as_str().chars(){
                            match ch {
                                ' '|'\n'=>{
                                    if !last_space{
                                        definition.push(' ');
                                    }
                                    last_space=true;
                                }
                                _=>{
                                    definition.push(ch);
                                    last_space=false
                                },
                            }
                        }
                        while let Some(c) = definition.pop(){
                            if c != ' ' && c != '\n' && c != '\t'{
                                definition.push(c);
                                break;
                            }
                        }
                        record.definition=Some(definition)
                    }
                }
                Rule::keyword=> {
                    if options.contains(&ParseOpts::Full)
                        || options.contains(&ParseOpts::Keywords)
                        || options.contains(&ParseOpts::FullHeader) {
                        let mut keywords = if let Some(kw) = record.keywords.take(){
                            kw
                        }else{
                            HashSet::new()
                        };
                        let mut kword = String::new();
                        for ch in field.as_str().chars() {
                            match ch {
                                '\n' => kword.push(' '),
                                '.' | '!' | '?' => (),
                                _ => kword.push(ch),
                            }
                        }
                        keywords.insert(kword);
                        record.keywords = Some(keywords);
                    }
                }
                Rule::dblink=>{
                    if options.contains(&ParseOpts::Full)
                        || options.contains(&ParseOpts::DBLinks)
                        || options.contains(&ParseOpts::FullHeader) {
                        let mut dblink = field.into_inner();
                        let key = dblink.next().unwrap();
                        let value = dblink.next().unwrap();
                        let mut dblinks = if let Some(dblinks) = record.dblink.take(){
                            dblinks
                        }else{
                            HashMap::new()
                        };
                        dblinks.insert(key.as_str().to_string(),value.as_str().to_string());
                        record.dblink = Some(dblinks);
                    }
                }
                Rule::src_def=>{
                    if options.contains(&ParseOpts::Full)
                        || options.contains(&ParseOpts::Source)
                        || options.contains(&ParseOpts::FullHeader) {
                        let mut source=String::new();
                        let mut last_space=false;
                        for ch in field.as_str().chars(){
                            match ch{
                                '\n' => {
                                    if !last_space{
                                        source.push(' ')
                                    }
                                },
                                ' '=>{
                                    if !last_space{
                                        source.push(' ')
                                    }
                                    last_space=true;
                                }
                                _=> {
                                    last_space=false;
                                    source.push(ch);
                                },
                            }
                        }
                        record.source=Some(source)
                    }
                }
                Rule::sci_name=> {
                    if options.contains(&ParseOpts::Full)
                        || options.contains(&ParseOpts::Organism)
                        || options.contains(&ParseOpts::FullHeader) {
                        record.organism = Some(field.as_str().to_string());
                    }
                }
                Rule::taxonomy=>{
                    if options.contains(&ParseOpts::Full)
                        || options.contains(&ParseOpts::Taxonomy)
                        || options.contains(&ParseOpts::FullHeader) {
                        let mut taxonomy = if let Some(tax)= record.taxonomy.take(){
                            tax
                        }else{
                            HashSet::new()
                        };
                        for taxon in field.into_inner() {
                            taxonomy.insert(taxon.as_str().trim().to_string().to_lowercase());
                        }
                        record.taxonomy= Some(taxonomy)
                    }
                }
                Rule::reference=>{
                    if options.contains(&ParseOpts::Full)
                        || options.contains(&ParseOpts::References)
                        || options.contains(&ParseOpts::FullHeader) {
                        let mut references = if let Some(refs)=record.references.take(){
                            refs
                        }else{
                            Vec::new()
                        };
                        references.push(Reference::from(field));
                        record.references=Some(references);
                    }
                }
                Rule::comment=>{
                    if options.contains(&ParseOpts::Full)
                        || options.contains(&ParseOpts::Comments)
                        || options.contains(&ParseOpts::FullHeader) {
                        let mut last_space = false;
                        let mut commt = String::new();
                        for ch in field.as_str().chars() {
                            match ch {
                                ' ' | '\n' => {
                                    if !last_space {
                                        commt.push(' ');
                                    }
                                    last_space = true;
                                }
                                _ => {
                                    commt.push(ch);
                                    last_space = false
                                },
                            }
                        }
                        while let Some(c) = commt.pop() {
                            if c != ' ' && c != '\n' && c != '\t' {
                                commt.push(c);
                                break;
                            }
                        }
                        record.comment = Some(commt)
                    }
                }
                _=>()
            }
        }
    }
    pub(crate) fn fill_features(features:Pair<Rule>, record: &mut GBRecord, options: &HashSet<ParseOpts>){
        if options.contains(&ParseOpts::Features) || options.contains(&ParseOpts::Full){
            let mut feat = if let Some(feat)=record.features.take(){
                feat
            }else{
                Vec::new()
            };
            for feature in features.into_inner() {
                feat.push(Feature::from(feature));
            }
            record.features=Some(feat);
        }
    }
    pub(crate) fn fill_origin(origin:Pair<Rule>, record:&mut GBRecord, options:&HashSet<ParseOpts>){
        if options.contains(&ParseOpts::Sequence) || options.contains(&ParseOpts::Full){
            let mut sequence = if let Some(seq)=record.sequence.take(){
                seq
            }else{
                String::new()
            };
            for seq_block in origin.into_inner() {
                sequence.push_str(seq_block.as_str().trim());
            }
            record.sequence=Some(sequence)
        }
    }
}