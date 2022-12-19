mod assembler;
mod lexical_analyzer;
mod peek_wrapper;
mod gen_errors;

use std::result;

use crate::lexical_analyzer::LexicalAnalyzer;
use crate::assembler::Assembler;
use crate::peek_wrapper::PeekWrapper;
use crate::gen_errors::GeneralError;

fn main() {

    //let mut analyzer = LexicalAnalyzer::new("file.txt".to_string(),true).unwrap();

    let mut assembler = Assembler::new("file.txt", "a").unwrap();

    let result = assembler.run();

    match result 
    {
        Err(err) =>
        {
            println!("ERROR:");
            println!("{}", err.to_string());
        },

        _ => {println!("Success!")}
    }
}
