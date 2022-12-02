mod instruction;

use std::collections::HashMap;
use crate::lexical_analyzer;
use crate::lexical_analyzer::LexicalAnalyzer;
use crate::assembler::instruction::Instruction;
use crate::lexical_analyzer::TokenType;
use crate::lexical_analyzer::Token;
use crate::peek_wrapper::PeekWrapper;
use crate::gen_errors::GeneralError;

// holds the assembler main struct 
pub struct Assembler
{
    lexical_analyzer: LexicalAnalyzer,
    symbol_table: HashMap<String, u32>,
    current_byte: u32,
    instruction_table: HashMap<String,Instruction>,
}


impl Assembler
{
    // new 
    // return a new assembler 
    pub fn new(file_name: String) -> Assembler
    {
        Assembler 
        {
            lexical_analyzer: LexicalAnalyzer::new(file_name, true).unwrap(),
            symbol_table: HashMap::new(),
            current_byte: 0,
            instruction_table: Instruction::get_map(),
        }
    }

    // run 
    // runs the assembler 
    pub fn run(& mut self ) -> Result<(),GeneralError>
    {

        self.first_pass()?;

        Ok(())
    }


    // first_pass
    // finds all the labels on logical lines 
    fn first_pass(& mut self) ->Result<(),GeneralError>
    {
        loop 
        {   
            // create iter
            let mut iter = PeekWrapper::new(self.lexical_analyzer.get_iterator(), 3);

            let token_result_option = iter.next();
            let token;

            // unwrap the option and get the token
            match token_result_option
            {
                None => break,
                Some(v)=> token=v?,
            }

            // we now have the token 
            


        }

        Ok(())
    }

}

