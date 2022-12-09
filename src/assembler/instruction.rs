
use std::collections::HashMap;
use crate::lexical_analyzer::TokenType;
use crate::lexical_analyzer::TokenType::*;

mod grammars{
    use crate::lexical_analyzer::TokenType;
    use crate::lexical_analyzer::TokenType::*;

    pub const IMMEDIAT2BYTE:[TokenType; 3] = [Hash, Num2Bytes, EOL];
    pub const ZEROPAGE:     [TokenType; 2] = [Num2Bytes, EOL];
    pub const ZEROPAGEX:    [TokenType; 4] = [Num2Bytes, Comma, RegX, EOL];
    pub const ABSOLUTE:     [TokenType; 2] = [Num4Bytes, EOL];
    pub const ABSOLUTEX:    [TokenType; 4] = [Num4Bytes, Comma, RegX, EOL]; 
    pub const ABSOLUTEY:    [TokenType; 4] = [Num4Bytes, Comma, RegY, EOL]; 
    pub const INDIRECTX:    [TokenType; 6] = [LeftParenth, Num2Bytes, Comma, RegX, RightParenth, EOL];  
    pub const INDIRECTY:    [TokenType; 6] = [LeftParenth, Num2Bytes, RightParenth, Comma, RegY, EOL];    
}

// instruction 
// holds a sring code i.e. "str", "and", etc.
// holds a hashmap full of possible grammars that this could be 
pub struct Instruction 
{
    pub string_code: String,
    pub opcode_grammer: Vec<(u8, Vec<TokenType>)>
}



impl Instruction 
{
    // get_map
    // returns a list 
    // returns a map of instruction string codes 
    // paired with their instruction struct  
    pub fn get_map() -> HashMap<String,Instruction>
    {
        let mut map: HashMap<String,Instruction> = HashMap::new();

        // lda 
        map.insert("lda".to_string(), Instruction{
            string_code:"lda".to_string(),
            opcode_grammer: vec![
                (0xa9,  grammars::IMMEDIAT2BYTE.to_vec()),
                (0xa5,  grammars::ZEROPAGE.to_vec()),
                (0xb5,  grammars::ZEROPAGEX.to_vec()),
                (0xad,  grammars::ABSOLUTE.to_vec()),
                (0xbd,  grammars::ABSOLUTEX.to_vec()),
                (0xb9,  grammars::ABSOLUTEY.to_vec()),
                (0xa1,  grammars::INDIRECTX.to_vec()),
                (0xb1,  grammars::INDIRECTY.to_vec()),
            
            ],
        });

        map
    }
}
