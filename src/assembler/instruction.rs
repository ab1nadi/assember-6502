
use std::collections::HashMap;
use crate::lexical_analyzer::TokenType;
use crate::lexical_analyzer::TokenType::*;

// instruction 
// holds a sring code i.e. "str", "and", etc.
// holds a hashmap full of possible grammars that this could be 
pub struct Instruction 
{
    string_code: String,
    opcode_grammer: HashMap<char,Vec<TokenType>>
}


// grammar
// holds a possible grammar for
// an instruction, it is a list of tokens that are expected 
pub struct Grammar<'a>
{
    gram: &'a Vec<TokenType>
}



impl Instruction 
{
    // get_map
    // returns a list 
    // returns a map of instruction string codes 
    // paired with their instruction struct  
    pub fn get_map()
    {

    }
}

impl<'a> Grammar<'a>
{
        pub const IMMEDIAT: [TokenType; 4] = [HexNum, DecNum, DecNum];
}
