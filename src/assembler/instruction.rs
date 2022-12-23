
use std::collections::HashMap;
use crate::assembler::lexical_analyzer::TokenType;

mod grammars{
    use crate::assembler::lexical_analyzer::TokenType;
    use crate::assembler::lexical_analyzer::TokenType::*;

    pub const IMMEDIAT1BYTE:[TokenType; 3] = [Hash, Num1Bytes, EOL];
    pub const ZEROPAGE:     [TokenType; 2] = [Num1Bytes, EOL];
    pub const ZEROPAGEX:    [TokenType; 4] = [Num1Bytes, Comma, RegX, EOL];
    pub const ZEROPAGEY:    [TokenType; 4] = [Num1Bytes, Comma, RegY, EOL];
    pub const ABSOLUTE:     [TokenType; 2] = [Num2Bytes, EOL];
    pub const ABSOLUTEX:    [TokenType; 4] = [Num2Bytes, Comma, RegX, EOL]; 
    pub const ABSOLUTEY:    [TokenType; 4] = [Num2Bytes, Comma, RegY, EOL]; 
    pub const INDIRECT:     [TokenType; 4] = [LeftParenth, Num2Bytes, RightParenth, EOL];
    pub const INDIRECTX:    [TokenType; 6] = [LeftParenth, Num1Bytes, Comma, RegX, RightParenth, EOL];  
    pub const INDIRECTY:    [TokenType; 6] = [LeftParenth, Num1Bytes, RightParenth, Comma, RegY, EOL];  
    pub const ACCUMULATOR:  [TokenType; 2] = [RegA, EOL]; 

    pub const EMPTY:        [TokenType; 1] = [EOL];
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
        // adc
        map.insert("adc".to_string(), Instruction{
            string_code:"adc".to_string(),
            opcode_grammer: vec![
                (0x69,  grammars::IMMEDIAT1BYTE.to_vec()),
                (0x65,  grammars::ZEROPAGE.to_vec()),
                (0x75,  grammars::ZEROPAGEX.to_vec()),
                (0x6d,  grammars::ABSOLUTE.to_vec()),
                (0x7d,  grammars::ABSOLUTEX.to_vec()),
                (0x79,  grammars::ABSOLUTEY.to_vec()),
                (0x61,  grammars::INDIRECTX.to_vec()),
                (0x71,  grammars::INDIRECTY.to_vec()),
            ],
        });


        // and
        map.insert("and".to_string(), Instruction{
            string_code:"and".to_string(),
            opcode_grammer: vec![
                (0x29,  grammars::IMMEDIAT1BYTE.to_vec()),
                (0x25,  grammars::ZEROPAGE.to_vec()),
                (0x35,  grammars::ZEROPAGEX.to_vec()),
                (0x2d,  grammars::ABSOLUTE.to_vec()),
                (0x3d,  grammars::ABSOLUTEX.to_vec()),
                (0x39,  grammars::ABSOLUTEY.to_vec()),
                (0x21,  grammars::INDIRECTX.to_vec()),
                (0x31,  grammars::INDIRECTY.to_vec()),
            ],
        });


        // asl 
        map.insert("asl".to_string(), Instruction{
            string_code:"asl".to_string(),
            opcode_grammer: vec![
                (0x0a,  grammars::ACCUMULATOR.to_vec()),
                (0x06,  grammars::ZEROPAGE.to_vec()),
                (0x16,  grammars::ZEROPAGEX.to_vec()),
                (0x0e,  grammars::ABSOLUTE.to_vec()),
                (0x1e,  grammars::ABSOLUTEX.to_vec()),
            ],
        });


        // Bit
        map.insert("bit".to_string(), Instruction{
            string_code:"bit".to_string(),
            opcode_grammer: vec![
                (0x24,  grammars::ZEROPAGE.to_vec()),
                (0x2c,  grammars::ABSOLUTE.to_vec()),
            ],
        });


        // brk
        map.insert("brk".to_string(), Instruction{
            string_code:"brk".to_string(),
            opcode_grammer: vec![
                (0x00, grammars::EMPTY.to_vec())
            ],
        });



         // cmp
         map.insert("cmp".to_string(), Instruction{
            string_code:"cmp".to_string(),
            opcode_grammer: vec![
                (0xc9,  grammars::IMMEDIAT1BYTE.to_vec()),
                (0xc5,  grammars::ZEROPAGE.to_vec()),
                (0xd5,  grammars::ZEROPAGEX.to_vec()),
                (0xcd,  grammars::ABSOLUTE.to_vec()),
                (0xdd,  grammars::ABSOLUTEX.to_vec()),
                (0xd9,  grammars::ABSOLUTEY.to_vec()),
                (0xc1,  grammars::INDIRECTX.to_vec()),
                (0xd1,  grammars::INDIRECTY.to_vec()),
            ],
        });


        // cpx
        map.insert("cpx".to_string(), Instruction{
            string_code:"cpx".to_string(),
            opcode_grammer: vec![
                (0xe0,  grammars::IMMEDIAT1BYTE.to_vec()),
                (0xe4,  grammars::ZEROPAGE.to_vec()),
                (0xec,  grammars::ABSOLUTE.to_vec()),
            ],
        });


        // cpy
        map.insert("cpy".to_string(), Instruction{
            string_code:"cpy".to_string(),
            opcode_grammer: vec![
                (0xc0,  grammars::IMMEDIAT1BYTE.to_vec()),
                (0xc4,  grammars::ZEROPAGE.to_vec()),
                (0xcc,  grammars::ABSOLUTE.to_vec()),
            ],
        });



        // dec
        map.insert("dec".to_string(), Instruction{
            string_code:"dec".to_string(),
            opcode_grammer: vec![
                (0xc6,  grammars::ZEROPAGE.to_vec()),
                (0xd6,  grammars::ZEROPAGEX.to_vec()),
                (0xc3,  grammars::ABSOLUTE.to_vec()),
                (0xde,  grammars::ABSOLUTEX.to_vec()),
            ],
        });


        // eor
        map.insert("eor".to_string(), Instruction{
            string_code:"eor".to_string(),
            opcode_grammer: vec![
                (0x49,  grammars::IMMEDIAT1BYTE.to_vec()),
                (0x45,  grammars::ZEROPAGE.to_vec()),
                (0x55,  grammars::ZEROPAGEX.to_vec()),
                (0x4d,  grammars::ABSOLUTE.to_vec()),
                (0x5d,  grammars::ABSOLUTEX.to_vec()),
                (0x59,  grammars::ABSOLUTEY.to_vec()),
                (0x41,  grammars::INDIRECTX.to_vec()),
                (0x51,  grammars::INDIRECTY.to_vec()),
            ],
        });


        // clc
        map.insert("clc".to_string(), Instruction{
            string_code:"clc".to_string(),
            opcode_grammer: vec![
                (0x18,  grammars::EMPTY.to_vec()),
            ],
        });

         // sec
         map.insert("sec".to_string(), Instruction{
            string_code:"sec".to_string(),
            opcode_grammer: vec![
                (0x38,  grammars::EMPTY.to_vec()),
            ],
        });


         // cli
         map.insert("cli".to_string(), Instruction{
            string_code:"cli".to_string(),
            opcode_grammer: vec![
                (0x58,  grammars::EMPTY.to_vec()),
            ],
        });


         // sei
         map.insert("sei".to_string(), Instruction{
            string_code:"sei".to_string(),
            opcode_grammer: vec![
                (0x78,  grammars::EMPTY.to_vec()),
            ],
        });


         // clv
         map.insert("clv".to_string(), Instruction{
            string_code:"clv".to_string(),
            opcode_grammer: vec![
                (0xb8,  grammars::EMPTY.to_vec()),
            ],
        });


         // cld
         map.insert("cld".to_string(), Instruction{
            string_code:"cld".to_string(),
            opcode_grammer: vec![
                (0xd8,  grammars::EMPTY.to_vec()),
            ],
        });

         // sed
         map.insert("sed".to_string(), Instruction{
            string_code:"sed".to_string(),
            opcode_grammer: vec![
                (0xf8,  grammars::EMPTY.to_vec()),
            ],
        });

        // inc
        map.insert("inc".to_string(), Instruction{
            string_code:"inc".to_string(),
            opcode_grammer: vec![
                (0xe6,  grammars::ZEROPAGE.to_vec()),
                (0xf6,  grammars::ZEROPAGEX.to_vec()),
                (0xee,  grammars::ABSOLUTE.to_vec()),
                (0xfe,  grammars::ABSOLUTEX.to_vec()),
            ],
        });


        
        // jmp
        map.insert("jmp".to_string(), Instruction{
            string_code:"jmo".to_string(),
            opcode_grammer: vec![
                (0x4c,  grammars::ABSOLUTE.to_vec()),
                (0x6c,  grammars::INDIRECT.to_vec()),
            ],
        });
        

        // jsr
        map.insert("jsr".to_string(), Instruction{
            string_code:"jsr".to_string(),
            opcode_grammer: vec![
                (0x20,  grammars::ABSOLUTE.to_vec()),
            ],
        });
        

        // lda 
        map.insert("lda".to_string(), Instruction{
            string_code:"lda".to_string(),
            opcode_grammer: vec![
                (0xa9,  grammars::IMMEDIAT1BYTE.to_vec()),
                (0xa5,  grammars::ZEROPAGE.to_vec()),
                (0xb5,  grammars::ZEROPAGEX.to_vec()),
                (0xad,  grammars::ABSOLUTE.to_vec()),
                (0xbd,  grammars::ABSOLUTEX.to_vec()),
                (0xb9,  grammars::ABSOLUTEY.to_vec()),
                (0xa1,  grammars::INDIRECTX.to_vec()),
                (0xb1,  grammars::INDIRECTY.to_vec()),
            
            ],
        });


         // ldx
         map.insert("ldx".to_string(), Instruction{
            string_code:"ldx".to_string(),
            opcode_grammer: vec![
                (0xa2,  grammars::IMMEDIAT1BYTE.to_vec()),
                (0xa6,  grammars::ZEROPAGE.to_vec()),
                (0xb6,  grammars::ZEROPAGEY.to_vec()),
                (0xae,  grammars::ABSOLUTE.to_vec()),
                (0xbe,  grammars::ABSOLUTEY.to_vec()),
            ],
        });

         // ldy
         map.insert("ldy".to_string(), Instruction{
            string_code:"ldy".to_string(),
            opcode_grammer: vec![
                (0xa0,  grammars::IMMEDIAT1BYTE.to_vec()),
                (0xa4,  grammars::ZEROPAGE.to_vec()),
                (0xb4,  grammars::ZEROPAGEX.to_vec()),
                (0xac,  grammars::ABSOLUTE.to_vec()),
                (0xbc,  grammars::ABSOLUTEX.to_vec()),
            ],
        });



        // lsr
        map.insert("ldy".to_string(), Instruction{
            string_code:"ldy".to_string(),
            opcode_grammer: vec![
                (0x4a,  grammars::ACCUMULATOR.to_vec()),
                (0x46,  grammars::ZEROPAGE.to_vec()),
                (0x56,  grammars::ZEROPAGEX.to_vec()),
                (0x4e,  grammars::ABSOLUTE.to_vec()),
                (0x5e,  grammars::ABSOLUTEX.to_vec()),
            ],
        });





        map
    }
}
