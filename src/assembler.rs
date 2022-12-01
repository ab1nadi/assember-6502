mod instruction;

use std::collections::HashMap;
use crate::lexical_analyzer::LexicalAnalyzer;
use crate::assembler::instruction::Instruction;
// holds the assembler main struct 
pub struct Assembler 
{
    lexical_analyzer: LexicalAnalyzer,
    symbol_table: HashMap<String, u32>,
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
        }
    }


}

