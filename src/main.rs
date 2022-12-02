mod assembler;
mod lexical_analyzer;
mod peek_wrapper;
use crate::lexical_analyzer::LexicalAnalyzer;
use crate::assembler::Assembler;
use crate::peek_wrapper::PeekWrapper;

fn main() {

    let mut analyzer = LexicalAnalyzer::new("file.txt".to_string(),true).unwrap();
   
    let  mut peek = PeekWrapper::new(analyzer.get_iterator(), 3);

    loop    {
        let i = peek.next();
        println!("{:?}", i);
        println!("peeking ahead: {:?}", peek.peek(0));

        match i
        {
            None => break,
            Some(_) => {},
        }
    }

}
