use crate::gbrecord::GBRecord;
use crate::features::Feature;
use std::option::Option::Some;

pub enum OutFmt{
    Fasta,
    GB
}

impl GBRecord {
    pub fn format_feature(&self, feature:&Feature, format:OutFmt)->String{
        match format {
            OutFmt::Fasta=>{
                format!(">{}_{}_{}\n{}\n", self.accession.as_ref().unwrap(),self.organism.as_ref().unwrap().replace(" ","_"), {
                    if let Some(name) = feature.properties.get("gene"){
                        name
                    }else if let Some(name) = feature.properties.get("product"){
                        name
                    }else if let Some(name) = feature.properties.get("note"){
                        name
                    }else{
                        " "
                    }
                },
                    self.extract_region(&feature.range)
                )
            }
            _=>unreachable!()
        }
    }
}