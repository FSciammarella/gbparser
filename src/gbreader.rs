use std::io::{BufReader, BufRead};
use std::fs::File;

use pest::Parser;

use crate::gbparser::*;
use crate::gbrecord::GBRecord;
use std::collections::HashSet;

#[derive(Debug)]
pub struct GBReader{
    file:BufReader<File>,
    buffer:String,
    leftovers:String,
    options:HashSet<ParseOpts>,
}
#[derive(Debug,Eq,PartialOrd, PartialEq,Hash)]
pub enum ParseOpts{
    Full,
    HoldBuffer,
    Alphabet,
    Section,
    Size,
    Definition,
    DBLinks,
    Source,
    Accession,
    Organism,
    Date,
    Features,
    References,
    Authors,
    Comments,
    Sequence,
    Taxonomy,
    FullHeader,
    Keywords,
    Version,
    Conformation,
}

impl GBReader{
    pub fn new(filename:&str, options:HashSet<ParseOpts> )->GBReader{
        GBReader{
            file: BufReader::new(File::open(filename).expect("Failed to open file")),
            buffer: String::new(),
            leftovers: String::new(),
            options
        }
    }
    fn fetch_next_record(&mut self)->Option<GBRecord>{
        let mut read_buf=String::new();
        let mut bar_count =0;
        let mut end = false;
        for chars in self.leftovers.chars(){
            self.buffer.push(chars);
        }
        self.leftovers.clear();
        while let Ok(n) = self.file.read_line(&mut read_buf){
            if n==0||end{
                break;
            }
            for chars in read_buf.chars(){
                match chars{
                    '/' =>{ bar_count+=1;
                    }
                    '\n'=>{
                        if bar_count==2{
                            end = true;
                        }
                    }
                    _=>bar_count=0
                }
                if !end{
                    self.buffer.push(chars)
                }else{
                    self.leftovers.push(chars)
                };
            }
            read_buf.clear();
        }
        match GBParser::parse(Rule::record,&self.buffer){
            Ok(mut r)=>{
                //                println!("{:#?}",r);
                let res = GBRecord::from(r.next().unwrap(), &self.options);
                self.buffer.clear();
                Some(res)
            }
            Err(e)=>{
                eprintln!("{}",self.buffer);
                eprintln!("{}",e);
                eprintln!("{:?}",e);
                self.buffer.clear();
                None
            }
        }
    }
}

impl Iterator for GBReader{
    type Item = GBRecord;
    fn next(&mut self) -> Option<Self::Item> {
        self.fetch_next_record()
    }
}