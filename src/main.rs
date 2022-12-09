mod assembler;
mod lexical_analyzer;
mod peek_wrapper;
mod gen_errors;

use crate::lexical_analyzer::LexicalAnalyzer;
use crate::assembler::Assembler;
use crate::peek_wrapper::PeekWrapper;
use crate::gen_errors::GeneralError;

fn main() {

    let mut analyzer = LexicalAnalyzer::new("file.txt".to_string(),true).unwrap();

    for i in analyzer.get_iterator()
    {
        println!("{:?}", i);
    }

}
