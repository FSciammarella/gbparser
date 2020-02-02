use pest::Parser;
use pest_derive::Parser;

#[derive(Parser,Debug)]
#[grammar= "grammars/gb.pest"]
pub(crate) struct GBParser;