mod assembler;
mod lexical_analyzer;
mod peek_wrapper;
mod gen_errors;

use crate::assembler::Assembler;

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
