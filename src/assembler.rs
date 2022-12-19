mod instruction;

use std::any::Any;
use std::collections::HashMap;
use crate::gen_errors;
use crate::lexical_analyzer;
use crate::lexical_analyzer::LexicalAnalyzer;
use crate::assembler::instruction::Instruction;
use crate::lexical_analyzer::TokenType;
use crate::lexical_analyzer::Token;
use crate::peek_wrapper::PeekWrapper;
use crate::gen_errors::GeneralError;
use crate::lexical_analyzer::LexicalIterator;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

use std::u8;
use std::u16;

// holds the assembler main struct 
pub struct Assembler
{
    lexical_analyzer: LexicalAnalyzer,
    symbol_table: HashMap<String, u32>,
    current_byte: u32,
    instruction_table: HashMap<String,Instruction>,
    file_writer: File
}


impl Assembler
{
    // new 
    // return a new assembler 
    pub fn new(file_name: &str, output_file_name: &str) -> Result<Assembler, GeneralError>
    {
        let mut file_result = File::create(output_file_name);
        let mut file;

        match file_result 
        {
            Ok(f) => file = f,
            Err(err) => return Err(Assembler::create_empty_error(err.to_string().as_str()))
        }
        
        
        Ok(Assembler 
        {
            lexical_analyzer: LexicalAnalyzer::new(file_name.to_string(), true).unwrap(),
            symbol_table: HashMap::new(),
            current_byte: 0,
            instruction_table: Instruction::get_map(),
            file_writer: file,
        })
    }

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
    fn first_pass(&mut self) ->Result<(),GeneralError>
    {
        // the iterator that will be used
        let mut iter = PeekWrapper::new(self.lexical_analyzer.get_iterator(), 3);

        let mut current_byte = self.current_byte;
        loop 
        {   
            // peek the next token 
            let next_token_option = iter.peek(0);
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
                    Assembler::directive_parser(&mut iter, &mut self.symbol_table, None,&mut current_byte)?;
                }
                TokenType::Label => 
                {
                    Assembler::label_parser_first_pass(&mut self.symbol_table, &mut iter, current_byte)?;
                },
                TokenType::Instruction => 
                {
                    current_byte = current_byte + Assembler::instruction_parser( &mut self.symbol_table, &self.instruction_table,&mut iter, None)?;
                },
                TokenType::EOF =>
                {
                    break;
                }
                _ => {return Err(Assembler::create_error("Syntax Error", &token, vec![TokenType::Instruction, TokenType::Directive, TokenType::Label]))}
            }

        }

        println!("current_byte: {}", current_byte);

        Ok(())
    }

    // first_pass
    // finds all the labels on logical lines 
    fn second_pass(&mut self) ->Result<(),GeneralError>
    {
        // reset the lexical analyzer 
        // so we can do another pass
        self.lexical_analyzer.reset()?;

        // the iterator that will be used
        let mut iter = PeekWrapper::new(self.lexical_analyzer.get_iterator(), 3);


        println!(" in the second pass");
        let mut current_byte = self.current_byte;
        loop 
        {   
            // peek the next token 
            let next_token_option = iter.peek(0);
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
                    Assembler::directive_parser(&mut iter, &mut self.symbol_table, Some(& mut self.file_writer), &mut current_byte)?;
                }
                TokenType::Label => 
                {
                    Assembler::label_parser_second_pass(&mut self.symbol_table, &mut iter, current_byte)?;
                },
                TokenType::Instruction => 
                {
                    current_byte = current_byte + Assembler::instruction_parser( &mut self.symbol_table, &self.instruction_table,&mut iter, Some(& mut self.file_writer))?;
                },
                TokenType::EOF =>
                {
                    break;
                }
                _ => {return Err(Assembler::create_error("Syntax Error", &token, vec![TokenType::Instruction, TokenType::Directive, TokenType::Label]))}
            }

        }

        println!("current_byte: {}", current_byte);

        Ok(())
    }

    
    // directive_parser 
    // does whatever the directive is supposed to do
    fn directive_parser(iterator: &mut PeekWrapper<LexicalIterator>, symbol_table: & mut HashMap<String, u32>, file_writer: Option<&mut File>, current_byte: &mut u32)-> Result<u32,GeneralError>
    {
        // the returned number of bytes
        // essentially a directive parser 
        // will return 0 if it isn't the directive 
        let mut returned_bytes = 0;

        returned_bytes = returned_bytes + Assembler::byte_directive_parser(iterator, symbol_table, file_writer)?;

        returned_bytes = returned_bytes + Assembler::org_directive_parser(iterator,current_byte)?;

        Ok(returned_bytes)
    }


    // label_parser_first_pass
    // adds a label to the symbol table
    fn label_parser_first_pass( symbol_table: &mut HashMap<String,u32>, iterator: &mut PeekWrapper<LexicalIterator>, current_byte: u32) -> Result<(),GeneralError>
    {
            // get the label
            let next_token_option =  iterator.next();
            let token_label:Token;
            match next_token_option 
            {
                None => {return Err(Assembler::create_empty_error("Something bad happened inside the assembler"))},
                Some(t) => token_label = t?,
            }


            // consume a colon if it is there 
            Assembler::consume_if_available(TokenType::Collon, iterator)?;

            // consume an eol if it is there 
            Assembler::consume_if_available(TokenType::EOL, iterator)?;


            // add the label to the symbol table 
            // if the label already exists throw an error 
           let option =  symbol_table.insert(token_label.value.clone(), current_byte);

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

    // label_parser_second_pass
    // basically just consumes a label
    // because it should already be on the symbol table for the
    // second pass
    fn label_parser_second_pass( symbol_table: &mut HashMap<String,u32>, iterator: &mut PeekWrapper<LexicalIterator>, current_byte: u32) -> Result<(),GeneralError>
    {
            // consume a label, it should be there
            Assembler::consume_if_available(TokenType::Label, iterator)?;

            // consume a colon if it is there 
            Assembler::consume_if_available(TokenType::Collon, iterator)?;

            // consume an eol if it is there 
            Assembler::consume_if_available(TokenType::EOL, iterator)?;

            Ok(())


    }

    // instruction_parser_first_pass
    // essentially this parses an instruction
    // from the lexical analyzer
    fn instruction_parser(symbol_table: &mut HashMap<String,u32>, instruction_table: &HashMap<String,Instruction>, iterator: &mut PeekWrapper<LexicalIterator>, file_writer: Option<&mut File>)-> Result<u32,GeneralError>
    {

        // returned number of bytes 
        let mut returned_bytes = 1;

        // get the instruction_data_structure
        let instruction_token = Assembler::unwrap_token_option(iterator.next(), iterator)?;
        let instruction_option = instruction_table.get(&instruction_token.value);
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
        let mut best_match:&(u8,Vec<TokenType>) = &instruction_data_struct.opcode_grammer[0];
 
        // holds a count of the number of elements that match
        let mut best_match_size: u32 = 0;

        // after everything is said and done did we get a match
        let mut got_a_match = false;


        // iterate over all the token grammars
        for grammar_vec in &instruction_data_struct.opcode_grammer
        {
            let mut matched = true;
            let mut match_count = 0;
            let mut total_bytes =  0;

            // iterate over all the possible tokens in a grammar
            for (i, token_type_grammar) in grammar_vec.1.iter().enumerate()
            {


                // only get tokens when we need to
                if (gotten_tokens.len() as i32)-1 < i as i32
                {
                    gotten_tokens.push(Assembler::unwrap_token_option(iterator.next(),iterator)?);
                }

                // allow a type coercion from label to 2bytes num because, ultimately,  thats what labels are
                if gotten_tokens[i].token_type == TokenType::Label && *token_type_grammar == TokenType::Num1Bytes
                {
                    match_count = match_count +1;

                }
                // no cohersion so just check if they equal or not
                else if gotten_tokens[i].token_type != *token_type_grammar
                {
                    matched = false;
                    break;
                }
                else 
                {
                    // if the current token is a number it needs to be written to output so we
                    // add it to the byte count
                    if (gotten_tokens[i].token_type == TokenType::Label && *token_type_grammar == TokenType::Num2Bytes) || gotten_tokens[i].token_type == TokenType::Num2Bytes
                    {
                        total_bytes = total_bytes + 1;
                    }
                    else if gotten_tokens[i].token_type == TokenType::Num2Bytes 
                    {
                        total_bytes = total_bytes + 2;
                    }


                    match_count = match_count +1;
                }
            }   


            // if we matched totally this is what we want it to be 
            if(matched)
            {
             
                returned_bytes = returned_bytes + total_bytes;
                best_match = &grammar_vec;
                got_a_match = true;
                break;
            }
            else  
            {
               if match_count > best_match_size
                    {
                        best_match_size = match_count;
                        best_match = &grammar_vec;
                    }

            }
        }
        

        // it matched something 
        if got_a_match
        {

            // if something matched and the file writer exists
            // write these bytes to the file 
            match file_writer
            {
                Some(f) => 
                {
                    // write the opcode
                    f.write(&[best_match.0]).unwrap();

                    // write the tokens
                    for token in gotten_tokens
                    {
                        println!("wrote: {:?}", token);
                        Assembler::write_token_to_file(f, token, symbol_table)?;
                    }
                }

                _ => {},
            }






            Ok(returned_bytes)
        }
        // nothing matched
        else
        {
            let top_token_option = gotten_tokens.get(best_match_size as usize);
            let top_token;
            match top_token_option
            {
                None => 
                { 
                    return Err(Assembler::create_empty_error("Something broke in instruction parser function"));
                },
                Some(s) => {top_token=s},
            }

            return Err(Assembler::create_error("Syntax Error", top_token, vec![best_match.1[(best_match_size) as usize]]));
        }

       
        
    }

    // unwrap_token_option
    // this function unwraps a token option
    // and creates an error if it gets nothing
    // needs the lexical_analyzer to get the linenumber of the error
    fn unwrap_token_option(token:Option<Result<Token,GeneralError>>, iterator: &mut PeekWrapper<LexicalIterator>)->Result<Token,GeneralError>
    {
        let instrucion_token;
        match token
        {
            None=>{ return Err(Assembler::create_error("Syntax Error, unpresidented eof. Or some other goofy error", &Token { token_type: TokenType::EOF, value: "".to_string(), logical_line: 0, file_line: iterator.iterator.analyzer.file_line }, vec![]))},
            Some(S) => { instrucion_token = S;}
        }

        instrucion_token
    }


    // consume_if_available
    // consumes a token if it matches the type 
    // given 
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
    // creates a general error
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
    // writes to a given file 
    // a given token
    fn write_token_to_file(file:&mut File, token: Token, symbol_table: &mut HashMap<String, u32>,) -> Result<(), GeneralError>
    {   
        let mut result = Ok(0);
        match token.token_type
        {
            TokenType::Num1Bytes => 
            {
                result = file.write(&[Assembler::one_byte_num_string_to_int(token.value)]);
            },
            TokenType::Num2Bytes =>
            {
                // convert it to a two byte number
                let two_byte_num = Assembler::two_byte_num_string_to_int(token.value);

                // get the upper and lower bytes
                let lower_byte:u8 = two_byte_num as u8;
                let upper_byte:u8 = (two_byte_num >> 8) as u8;

                // since it is little endian we store the lower byte first
                result = file.write(&[lower_byte, upper_byte]);
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
                result = file.write(&[lower_byte, upper_byte]);
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
                file.write(&[character as u8]);
            }
            _ => { }
        }



        match result {
            Err(err)=> Err(Assembler::create_empty_error("Problem writing to file")),

            _=> Ok(())
        }


    }


    // one_byte_num_string_to_int
    // converts a one byte number
    // string to a u8
    fn one_byte_num_string_to_int(num: String) -> u8
    {

        let mut returned:u8 = 0;

        // its a hex number
        if num.chars().next().unwrap() == '$'
        {
            // get the string char iterator
            let mut it = num.chars();
            it.next();

            // get the rest of it as a str
            let hex_num_str = it.as_str();

            

            let hex_num = u8::from_str_radix(hex_num_str, 16).unwrap();


            println!("the hex_num_str: {}", hex_num);

            returned = hex_num;

        }
        // not hex
        else 
        {
            returned = num.parse().unwrap();
        }

        returned
    }

    // two_byte_num_string_to_int
    // converts a two byte number
    // string to a u8
    fn two_byte_num_string_to_int(num: String) -> u16
    {
        let mut returned:u16 = 0;

        // its a hex number
        if num.chars().next().unwrap() == '$'
        {
            // get the string char iterator
            let mut it = num.chars();
            it.next();

            // get the rest of it as a str
            let hex_num_str = it.as_str();

            let hex_num = u16::from_str_radix(hex_num_str, 16).unwrap();

            returned = hex_num;

        }
        // not hex
        else 
        {   
            // it won't cause an error
            // but on inputs greater than 2 bytes
            // it will only take the bottom 2 bytes
            returned = num.parse::<u32>().unwrap() as u16;
        }

        returned
    }


    

    // possible directives for the assembler 
    ////////////////////////////////////////////////////////////////////////
    /// 
    

    // byte_directive
    // accepts .byte or .BYTE 
    // and a list of bytes after it 
    // will store 2 bytes or 4 byte values 
    // witch can be labels, 
    fn byte_directive_parser(iterator: &mut PeekWrapper<LexicalIterator>, symbol_table: &mut HashMap<String, u32>, file_writer:Option<&mut File>)-> Result<u32,GeneralError>
    {
        // peek the token 
        let token_option = iterator.peek(0);
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
            iterator.next();

            let mut current_token = Assembler::unwrap_token_option(iterator.next(), iterator)?;

            let mut tokens: Vec<Token> = vec![];

            let mut returned_bytes = 0;

            // while not at the end of the line 
            while current_token.token_type != TokenType::EOL 
            {
                // consume a comma if availabe 
                Assembler::consume_if_available(TokenType::Comma, iterator)?;

                println!("{:?}", current_token);

                if current_token.token_type == TokenType::Character || current_token.token_type == TokenType::Num1Bytes 
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
                    return Err(Assembler::create_error("Syntax error", &current_token, vec![TokenType::Character, TokenType::Num1Bytes, TokenType::Num2Bytes, TokenType::Label]))
                }

                current_token = Assembler::unwrap_token_option(iterator.next(), iterator)?;
            }
            

            match file_writer
            {
                Some(f) => 
                {
                   for token in tokens 
                   {
                     Assembler::write_token_to_file(f, token, symbol_table)?;
                   }
                }

                _ => {},
            }

            

            Ok(returned_bytes)
        }
        else 
        {
            Ok(0)
        }

    }




    // org_directive_parser 
    // accepts .org or .ORG
    // will set the org 
    // of the current byte count
    // so that labels will be in relation to that 
    fn org_directive_parser(iterator: &mut PeekWrapper<LexicalIterator>, current_byte: &mut u32)-> Result<u32,GeneralError>
    {
         // peek the token 
         let token_option = iterator.peek(0);
         let token;
         match token_option 
         {
             None => return Err(Assembler::create_empty_error("Something bad happened in the byte_directive_parser")),
             Some(t)=> token = t?,
         }
 
         // this is the byte directive 
         if token.value.to_lowercase() == ".org"
         {
            // consume the .org
            iterator.next();

            // get the next token 
            let token = Assembler::unwrap_token_option(iterator.next(), iterator)?;

            if token.token_type == TokenType::Num1Bytes 
            {
                *current_byte = Assembler::one_byte_num_string_to_int(token.value) as u32;
            }
            else if token.token_type == TokenType::Num2Bytes
            {
                *current_byte = Assembler::two_byte_num_string_to_int(token.value) as u32;
            }
            else 
            {
                return Err(Assembler::create_error("Syntax error", &token, vec![TokenType::Num1Bytes, TokenType::Num2Bytes]))
            }


            Assembler::consume_if_available(TokenType::EOL, iterator)?;

         }


         // doesn't return a byte count
         Ok(0)

    }
    

}

