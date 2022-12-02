mod instruction;
mod errors;

use std::collections::HashMap;
use crate::lexical_analyzer::LexicalAnalyzer;
use crate::assembler::instruction::Instruction;
use crate::assembler::errors::AssemblerError;
use crate::lexical_analyzer::TokenType;
use crate::lexical_analyzer::Token;
// holds the assembler main struct 
pub struct Assembler
{
    lexical_analyzer: LexicalAnalyzer,
    symbol_table: HashMap<String, u32>,
    current_byte: u32,
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
        }
    }

    // run 
    // runs the assembler 
    pub fn run(& mut self ) -> Result<(),AssemblerError>
    {

        self.first_pass()?;

        Ok(())
    }


    // first_pass
    // finds all the labels on logical lines 
    fn first_pass(& mut self) ->Result<(),AssemblerError>
    {
        
        for tokenResult in &mut self.lexical_analyzer
        {   
            match tokenResult{
                Ok(Token) =>
                {
                    // there is three things we expect here 
                    // 1. directive
                    // 2. a label
                    // 3. a  instruction opcode
                    match Token.token_type
                    {
                        TokenType::Label =>
                        {

                        },
                        TokenType::Instruction =>
                        {

                        }
                        _=>
                        {

                        }
                    }
                },
                Err(err)=>
                {
                    return Err(AssemblerError::new(err.details.as_str()));
                }


            }
        }


        Ok(())
    }


    // first_pass_label
    // adds labels and their byte
    // values to the symbol table
    fn first_pass_label(token:Token)
    {

    }

    // first_pass_instruction
    // checks that instructions match
    // a given grammar     
    fn first_pass_instruction(token:Token)
    {

    }
}

