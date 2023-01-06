mod instruction;
mod lexical_analyzer;
mod peek_wrapper;
mod gen_errors;



extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}


// crate imports 
use crate::assembler::lexical_analyzer::LexicalAnalyzer;
use crate::assembler::instruction::Instruction;
use crate::assembler::lexical_analyzer::TokenType;
use crate::assembler::lexical_analyzer::Token;
use crate::assembler::peek_wrapper::PeekWrapper;
use crate::assembler::gen_errors::GeneralError;
use crate::assembler::lexical_analyzer::LexicalIterator;

// std imports
use std::collections::HashMap;

use std::u8;
use std::u16;

// holds the assembler main struct 
pub struct Assembler<'a>
{
    assembly_string: &'a String,
    lexical_iterator: PeekWrapper<LexicalIterator>,
    symbol_table: HashMap<String, u32>,
    current_byte: u32,
    instruction_table: HashMap<String,Instruction>,
    pub object_code: Vec<u8>
}


impl<'a> Assembler<'a>
{
    // new 
    // return a new assembler 
    pub fn new(assembly_string:&mut String) -> Result<Assembler, GeneralError>
    {
     
        Ok(Assembler 
        {
            assembly_string: assembly_string,
            lexical_iterator: PeekWrapper::new(LexicalAnalyzer::new( assembly_string.to_string(), true)?.get_iterator(),3),
            symbol_table: HashMap::new(),
            current_byte: 0,
            instruction_table: Instruction::get_map(),
            object_code: vec![],
        })
    }

    // get_obj_str
    // gets the object code as a string
    // also deletes the file

    // run 
    // runs the assembler 
    pub fn run(& mut self ) -> Result<(),GeneralError>
    {
        self.first_pass()?;
        self.second_pass()?;

        Ok(())
    }

    // first_pass
    // finds all the labels on logical lines 
    // while checking syntax
    fn first_pass(&mut self) ->Result<(),GeneralError>
    {
        loop 
        {   
            // peek the next token 
            let next_token_option = self.lexical_iterator.peek(0);
            let token:Token;
            match next_token_option 
            {
                None => break,
                Some(t) => token = t?,
            }

            match token.token_type
            {   
                
                TokenType::Directive => 
                {
                    Assembler::directive_parser(self, true)?;
                }
                TokenType::Label => 
                {
                    Assembler::label_parser(self, true)?;
                },
                TokenType::Instruction => 
                {
                  Assembler::instruction_parser( self, true)?;
                },
                TokenType::EOF =>
                {
                    break;
                }
                _ => {return Err(Assembler::create_error("Syntax Error", &token, vec![TokenType::Instruction, TokenType::Directive, TokenType::Label]))}
            }

        }


        Ok(())
    }

    // first_pass
    // checks syntax while writting everything
    // to file
    fn second_pass(&mut self) ->Result<(),GeneralError>
    {
        
        // reset the lexical analyzer 
        // so we can do another pass
        self.lexical_iterator = PeekWrapper::new(LexicalAnalyzer::new(self.assembly_string.to_string(), true).unwrap().get_iterator(),3);
        
        self.current_byte = 0;

        loop 
        {   
            // peek the next token 
            let next_token_option = self.lexical_iterator.peek(0);
            let token:Token;
            match next_token_option 
            {
                None => break,
                Some(t) => token = t?,
            }

            match token.token_type
            {   
                TokenType::Directive =>
                {
                    Assembler::directive_parser(self, false)?;
                }
                TokenType::Label => 
                {
                    Assembler::label_parser(self, false)?;
                },
                TokenType::Instruction => 
                {
                  Assembler::instruction_parser( self, false)?;
                },
                 TokenType::EOF =>
                {
                    break;
                }
                _ => {return Err(Assembler::create_error("Syntax Error", &token, vec![TokenType::Instruction, TokenType::Directive, TokenType::Label]))}
            }

        }


        Ok(())
    }

    // directive_parser 
    // parses directives if the directive
    // given isn't an implemented directive it throws an error
    fn directive_parser(assembler: &mut Assembler, first_pass: bool)-> Result<(),GeneralError>
    {
        let token = assembler.lexical_iterator.peek(0).unwrap()?;

        let mut _parsed_something = false;
        _parsed_something = _parsed_something || Assembler::byte_directive_parser(assembler, first_pass)?;
        _parsed_something = _parsed_something || Assembler::org_directive_parser(assembler)?;


        // it didn't parse anything 
        if !_parsed_something 
        {
            return Err(Assembler::create_error("Syntax Error, directive not implemented", &token, vec![TokenType::Directive]));
        }

        Ok(())
    }


    // label_parser_first_pass
    // adds a label to the symbol table
    fn label_parser(assembler: &mut Assembler, first_pass:bool) -> Result<(),GeneralError>
    {
            // don't do anything on the second pass
            if !first_pass 
            {
                 // consume a colon if it is there 
                 Assembler::consume_if_available(TokenType::Label, &mut assembler.lexical_iterator)?;

                // consume a colon if it is there 
                Assembler::consume_if_available(TokenType::Collon, &mut assembler.lexical_iterator)?;

                // consume an eol if it is there 
                Assembler::consume_if_available(TokenType::EOL, &mut assembler.lexical_iterator)?;

                return Ok(());
            }

            // get the label
            let next_token_option =  assembler.lexical_iterator.next();
            let token_label:Token;
            match next_token_option 
            {
                None => {return Err(Assembler::create_empty_error("Something bad happened inside the assembler"))},
                Some(t) => token_label = t?,
            }


            // consume a colon if it is there 
            Assembler::consume_if_available(TokenType::Collon, &mut assembler.lexical_iterator)?;

            // consume an eol if it is there 
            Assembler::consume_if_available(TokenType::EOL, &mut assembler.lexical_iterator)?;


            // add the label to the symbol table 
            // if the label already exists throw an error 
           let option =  assembler.symbol_table.insert(token_label.value.clone(), assembler.current_byte);

           match option
           {
                None => {},

                // throw an error the label already exists
                Some(t)=> 
                {
                    let error_des = format!("{}:Semantic Error, label {{ {} }} is already defined at line: {}",token_label.file_line, token_label.value, t);
                    return Err(Assembler::create_empty_error(error_des.as_str()));
                }
           }

            Ok(())


    }

    // instruction_parser
    // essentially this parses an instruction
    // from the lexical analyzer and writes it to file
    // if first_pass=false, replaces labels with their value on the symbol table
    // TODO: clean up this code a little
    fn instruction_parser(assembler: &mut Assembler, first_pass: bool)-> Result<(),GeneralError>
    {

   
        // get the instruction_data_structure
        let instruction_token = Assembler::unwrap_token_option(assembler.lexical_iterator.next(), &mut assembler.lexical_iterator)?;
        let instruction_option = assembler.instruction_table.get(&instruction_token.value.to_lowercase());
        let instruction_data_struct;
        // unwrap the instruction_option
        match instruction_option
        {
            None=>{return Err(Assembler::create_error("Instruction has not been implemented yet!", &instruction_token, vec![]))},
            Some(t)=>{instruction_data_struct = t},
        }

        // gotten_tokens holds the gotten tokens
        let mut gotten_tokens:Vec<Token> = vec![];

        // holds a ref to the bestmatching grammar
        let mut best_matching_grammar:&(u8,Vec<TokenType>) = &instruction_data_struct.opcode_grammer[0];
 
        // holds a count of the number of elements that match
        let mut best_match_count: i32 = 0;

        // iterate over all the token grammars
        for grammar_vec in &instruction_data_struct.opcode_grammer
        {
            let mut matched = true;

            for (i, grammar_token) in grammar_vec.1.iter().enumerate()
            {
               // get a token if we need to 
               if (gotten_tokens.len() as i32 -1) < i as i32
               {
                    if let Some(token_res) = assembler.lexical_iterator.next()
                    {
                        if let Err(e) = token_res
                        {
                            return Err(Assembler::create_empty_error("Something bad happened in instruction parser"));
                        }
                        else 
                        {
                            gotten_tokens.push(token_res.unwrap());   
                        }
                    }
                    else
                    {
                        matched=false;
                        break;
                    }
               }
                
                // if they don't equal it is time to exit
                if *grammar_token != gotten_tokens[i].token_type 
                {
                    matched = false;
                    break;
                }
                else
                {
                    // if i is greater than best_match_count we have a new best match
                    if i >= best_match_count as usize 
                    {
                        best_match_count = (i+1) as i32;
                        best_matching_grammar = grammar_vec;

                    }
                

                }

            }

            // they matched so exit
            if matched 
            {
                
                // write the opcode on second pass
                if !first_pass
                {                    
                    assembler.object_code.push(best_matching_grammar.0);
                }

                for token in gotten_tokens
                {

                    if token.token_type == TokenType::Num1Bytes
                    {
                        assembler.current_byte = assembler.current_byte +1;
                    }
                    else if token.token_type == TokenType::Num1Bytes
                    {
                        assembler.current_byte = assembler.current_byte +2;
                    }
                    else if token.token_type == TokenType::Label
                    {
                        assembler.current_byte = assembler.current_byte +2;
                    }

                    // write the operands on second pass
                    if !first_pass
                    {
                        Assembler::write_token_to_file(&mut assembler.object_code, token, &mut assembler.symbol_table)?;
                    }
                }
                
                return Ok(());
            }
        }

        
        // there was an error because we weren't supposed to make it this far
        return Err(Assembler::create_error("Syntax error", &gotten_tokens[gotten_tokens.len()-1], vec![best_matching_grammar.1[(best_match_count) as usize]]));
         

    }

    // unwrap_token_option
    // this function unwraps a token option
    // and creates an error if it gets nothing
    fn unwrap_token_option(token:Option<Result<Token,GeneralError>>, iterator: &mut PeekWrapper<LexicalIterator>)->Result<Token,GeneralError>
    {
        let instrucion_token;
        match token
        {
            None=>{ return Err(Assembler::create_error("Syntax Error, unpresidented eof. Or some other goofy error", &Token { token_type: TokenType::EOF, value: "".to_string(), logical_line: 0, file_line: iterator.iterator.analyzer.file_line }, vec![]))},
            Some(s) => { instrucion_token = s;}
        }

        instrucion_token
    }

    // consume_if_available
    // consumes a token if it matches the given TokenType 
    fn consume_if_available(token_type: TokenType, iterator: &mut PeekWrapper<LexicalIterator>)-> Result<(),GeneralError>
    {
        let next_token_option = iterator.peek(0);
        let token:Token ;
        match next_token_option 
        {
            None => {},
            Some(t) => {
                token = t?;

                if token.token_type == token_type                  
                {  
                    let ioption = iterator.next();
                    match ioption 
                    {
                        None=> { },

                        Some(token) => {
                                 token?;
                        },
                    }
                  }
            }
        }

        Ok(())
    }
    
    // cretae_error
    // with expected and recived tokens
    fn create_error(error_description:&str, recieved:&Token, expected:Vec<TokenType>) ->GeneralError
    {
        let mut expec= "[".to_string();
        for i in expected
        {
            expec = expec + &i.to_string() + ", ";
        }
        expec = expec + "]";

        let string = format!("{line}:{description}, expected: {expected:?}, recieved: {token}", line=recieved.file_line,description=error_description, expected=expec, token=recieved.to_string());

        GeneralError::new(&string,"Assembler")
    }

    // create_empty_error
    // doesn't have a recived or expected
    // this is used for errors that are assembler based
    // i.e. something broke in the assembler
    fn create_empty_error(error_description:&str) ->GeneralError
    {
        GeneralError::new(&error_description,"Assembler")
    }

    // write_to_file
    // writes to a given file a given token
    // does different things based on the token type
    fn write_token_to_file(object_code_vec:&mut Vec<u8>, token: Token, symbol_table: &mut HashMap<String, u32>,) -> Result<(), GeneralError>
    {   

        match token.token_type
        {
            TokenType::Num1Bytes => 
            {
                let i = Assembler::one_byte_num_string_to_int(token.value);

                object_code_vec.push(i)
            },
            TokenType::Num2Bytes =>
            {
                // convert it to a two byte number
                let two_byte_num = Assembler::two_byte_num_string_to_int(token.value);

                // get the upper and lower bytes
                let lower_byte:u8 = two_byte_num as u8;
                let upper_byte:u8 = (two_byte_num >> 8) as u8;

                // since it is little endian we store the lower byte first
                object_code_vec.push(lower_byte);
                object_code_vec.push(upper_byte);
            },
            TokenType::Label =>
            {
                // convert the label to a 2byte number
                let two_byte_num_option = symbol_table.get(&token.value);
                let two_byte_num;
                match two_byte_num_option 
                {
                    None => {
                        return Err(Assembler::create_error("Label doesn't exist", &token, vec![]))
                    }, 
                    Some(t) => two_byte_num = *t as u16,
                }

                // get the upper and lower bytes
                let lower_byte:u8 = two_byte_num as u8;
                let upper_byte:u8 = (two_byte_num >> 8) as u8;

                // since it is little endian we store the lower byte first
                object_code_vec.push(lower_byte);
                object_code_vec.push(upper_byte);
            },
            TokenType::Character =>
            {
                let mut iter = token.value.chars();

                // remove the front and back '
                iter.next();
                iter.next_back();

                // get the character
                let character = iter.next().unwrap();

                // write the character to the file
                object_code_vec.push(character as u8);
                
            },
            TokenType::String =>
            {   
                // get the string and remove the quotations ""
                let mut characters = token.value.chars();
                characters.next();
                characters.next_back();

                // write the string bytes 
                object_code_vec.append(& mut characters.as_str().to_string().as_bytes().to_vec());
            },
            _ => { 

            }
        }

            Ok(())

    }

    // one_byte_num_string_to_int
    // converts a one byte number
    // string to a u8
    fn one_byte_num_string_to_int(num: String) -> u8
    {

        let mut _returned:u8 = 0;

        // its a hex number
        if num.chars().next().unwrap() == '$'
        {
            // get the string char iterator
            let mut it = num.chars();
            it.next();

            // get the rest of it as a str
            let hex_num_str = it.as_str();

            

            let hex_num = u8::from_str_radix(hex_num_str, 16).unwrap();



            _returned = hex_num;

        }
        // not hex
        else 
        {
            _returned = num.parse().unwrap();
        }

        _returned
    }

    // two_byte_num_string_to_int
    // converts a two byte number
    // string to a u8
    fn two_byte_num_string_to_int(num: String) -> u16
    {
        let mut _returned:u16 = 0;

        // its a hex number
        if num.chars().next().unwrap() == '$'
        {
            // get the string char iterator
            let mut it = num.chars();
            it.next();

            // get the rest of it as a str
            let hex_num_str = it.as_str();

            let hex_num = u16::from_str_radix(hex_num_str, 16).unwrap();

            _returned = hex_num;

        }
        // not hex
        else 
        {   
            // it won't cause an error
            // but on inputs greater than 2 bytes
            // it will only take the bottom 2 bytes
            _returned = num.parse::<u32>().unwrap() as u16;
        }

        _returned
    }

    

    // possible directives for the assembler 
    ////////////////////////////////////////////////////////////////////////
    /// 
    
    // byte_directive
    // accepts .byte or .BYTE 
    // and a list of bytes after it 
    // will store 2 bytes or 4 byte values 
    // witch can be labels, 
    fn byte_directive_parser(assembler: &mut Assembler, first_pass:bool)-> Result<bool,GeneralError>
    {

        // peek the token 
        let token_option = assembler.lexical_iterator.peek(0);
        let token;
        match token_option 
        {
            None => return Err(Assembler::create_empty_error("Something bad happened in the byte_directive_parser")),
            Some(t)=> token = t?,
        }

        // this is the byte directive 
        if token.value.to_lowercase() == ".byte"
        {
            // consume the .byte
            // its there because we peeked it
            assembler.lexical_iterator.next();

            let mut current_token = Assembler::unwrap_token_option(assembler.lexical_iterator.next(), & mut assembler.lexical_iterator)?;

            let mut tokens: Vec<Token> = vec![];

            let mut returned_bytes = 0;

            // while not at the end of the line 
            while current_token.token_type != TokenType::EOL 
            {
                // consume a comma if availabe 
                Assembler::consume_if_available(TokenType::Comma, & mut assembler.lexical_iterator)?;

                 if current_token.token_type == TokenType::Character || current_token.token_type == TokenType::Num1Bytes || current_token.token_type == TokenType::String
                {
                    tokens.push(current_token);
                    returned_bytes = returned_bytes+1;
                }
                else if current_token.token_type == TokenType::Num2Bytes || current_token.token_type == TokenType::Label  
                {
                    tokens.push(current_token);
                    returned_bytes = returned_bytes+2;
                }
                else 
                {
                    return Err(Assembler::create_error("Syntax error", &current_token, vec![TokenType::Character, TokenType::Num1Bytes, TokenType::Num2Bytes, TokenType::Label, TokenType::String]))
                }

                current_token = Assembler::unwrap_token_option(assembler.lexical_iterator.next(), &mut assembler.lexical_iterator)?;
            }
            

           if !first_pass 
           {
                for token in tokens 
                {
                Assembler::write_token_to_file(&mut assembler.object_code, token, &mut assembler.symbol_table)?;
                }
           }


           assembler.current_byte = assembler.current_byte + returned_bytes;

           return Ok(true);

        }

        Ok(false)
    }

    // org_directive_parser 
    // accepts .org or .ORG
    // will set the org 
    // of the current byte count
    // so that labels will be in relation to that 
    fn org_directive_parser(assembler:&mut Assembler)-> Result<bool,GeneralError>
    {

         // peek the token 
         let token_option = assembler.lexical_iterator.peek(0);
         let token;
         match token_option 
         {
             None => return Err(Assembler::create_empty_error("Something bad happened in the org_directive_parser")),
             Some(t)=> token = t?,
         }
 
         // this is the byte directive 
         if token.value.to_lowercase() == ".org"
         {
            // consume the .org
            assembler.lexical_iterator.next();

            // get the next token 
            let token = Assembler::unwrap_token_option(assembler.lexical_iterator.next(), &mut assembler.lexical_iterator)?;

            if token.token_type == TokenType::Num1Bytes 
            {
                assembler.current_byte = Assembler::one_byte_num_string_to_int(token.value) as u32;
            }
            else if token.token_type == TokenType::Num2Bytes
            {
                assembler.current_byte = Assembler::two_byte_num_string_to_int(token.value) as u32;
            }
            else 
            {
                return Err(Assembler::create_error("Syntax error", &token, vec![TokenType::Num1Bytes, TokenType::Num2Bytes]))
            }


            Assembler::consume_if_available(TokenType::EOL, &mut assembler.lexical_iterator)?;

            return Ok(true);
         }


         // doesn't return a byte count
         Ok(false)

    }
    

}

