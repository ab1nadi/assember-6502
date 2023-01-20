mod instruction;
mod lexical_analyzer;
mod peek_wrapper;
mod gen_errors;
use std::num::Wrapping;
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
use std::fs::File;
use std::io::prelude::*;
use std::num;
use std::ptr::null;
use std::thread::current;
use std::u8;
use std::u16;

// holds the assembler main struct 
pub struct Assembler
{
    read_file_name: String,
    lexical_iterator: PeekWrapper<LexicalIterator>,
    symbol_table: HashMap<String,InsertableNum>,  
    current_byte: u32,
    instruction_table: HashMap<String,Instruction>,
    file_writer: File
}


#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]

pub enum InsertableNum
{
    Byte(u8),
    TwoByte(u16),
}

impl InsertableNum
{
    // unwrap
    // this is basically for printing purposes
    // unwraps it into a u32
    pub fn unwrap( self) -> u32
    {
        match self
        {
            InsertableNum::Byte(num) => num as u32,
            InsertableNum::TwoByte(num) => num as u32
        }
    }

    // unwrap_byte
    // returns a u8
    // will not cast this into u16s because we don't believe
    // in down casting
    pub fn unwrap_byte(self) -> u8
    {
        match self
        {
            InsertableNum::Byte(num) => num as u8,
            InsertableNum::TwoByte(num) => panic!("Down casting is frowned upon my dude")
        }
    }

    // unwrap_twobyte 
    // exptects it be two bytes
    // never panics because we believe in upcasting
    pub fn unwrap_twobyte(self) -> u16
    {
        match self
        {
            InsertableNum::TwoByte(num) => num as u16,
            InsertableNum::Byte(num) => num as u16
        }
    }


    // is_two_bytes 
    // returns true if this number is twobytes 
    pub fn is_two_bytes(self) -> bool 
    {
        match self
        {
            InsertableNum::TwoByte(_) => true,
            InsertableNum::Byte(_) => false
        }
    }
}


impl Assembler
{
    // new 
    // return a new assembler 
    pub fn new(file_name: &str, output_file_name: &str) -> Result<Assembler, GeneralError>
    {
        let  file_result = File::create(output_file_name);
        let  file;

        match file_result 
        {
            Ok(f) => file = f,
            Err(err) => return Err(Assembler::create_empty_error(err.to_string().as_str()))
        }
        
    
     
        Ok(Assembler 
        {
            read_file_name: file_name.to_string(),
            lexical_iterator: PeekWrapper::new(LexicalAnalyzer::new(file_name.to_string(), true)?.get_iterator(),3),
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

        println!("{:?}", self.symbol_table);
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
        self.lexical_iterator = PeekWrapper::new(LexicalAnalyzer::new(self.read_file_name.to_string(), true).unwrap().get_iterator(),3);
        
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

        // basically just consumes the tokens
        // because everything with labels is done in the first pass
        if !first_pass
        {
            Assembler::consume_if_available(TokenType::Label, &mut assembler.lexical_iterator)?;

            // look at the next token
            let token_option = assembler.lexical_iterator.peek(0);
            let token;
            // unwrap it 
            if let None = token_option
            {
                return Err(Assembler::create_empty_error("Something bad happened in the label parser on the second pass"));
            }
            else 
            {
                token = token_option.unwrap()?;
            }
            
            // this is a variable label it points to a variable
            if  token.token_type == TokenType::EQUALS
            {       
                // consume until eof 
                let mut current_token = token;
                while current_token.token_type != TokenType::EOL
                {
                    let optional_token = assembler.lexical_iterator.next();
                    if let None = optional_token
                    {
                        return Err(Assembler::create_empty_error("Something bad happened in the label parser on the second pass"));
                    }
                    else 
                    {
                        current_token = optional_token.unwrap()?;
                    }
                }
            }
            // this is a normal label that points to a place in code or memory
            else
            {
                Assembler::consume_if_available(TokenType::Collon, &mut assembler.lexical_iterator)?;
                Assembler::consume_if_available(TokenType::EOL, &mut assembler.lexical_iterator)?;
            }

            return Ok(())
        }


        // get the label
        let label_token = assembler.lexical_iterator.next().unwrap()?; // we can unwrap because we know it is there 

        // peek the next token 
        // look at the next token
        let token_option = assembler.lexical_iterator.peek(0);
        let next_token;
        // unwrap it 
        if let None = token_option
        {
            return Err(Assembler::create_empty_error("Something bad happened in the label parser on the first pass"));
        }
        else 
        {
            next_token = token_option.unwrap()?;
        }

        let mut label_num_value:InsertableNum = InsertableNum::Byte(0); // just initializing it to 0 
        
        if next_token.token_type == TokenType::EQUALS
        {
            // consume the equals
            assembler.lexical_iterator.next();

            // set the label_num_value to whatever the
            // expression paser finds
            label_num_value = Assembler::expression_parser(assembler, true)?;
            Assembler::consume_if_available(TokenType::EOL, &mut assembler.lexical_iterator)?;


        }
        else 
        {
            // get the optional characters : and eol
            Assembler::consume_if_available(TokenType::Collon, &mut assembler.lexical_iterator)?;
            Assembler::consume_if_available(TokenType::EOL, &mut assembler.lexical_iterator)?;

            // insert the num
            label_num_value = InsertableNum::TwoByte(assembler.current_byte as u16);
        }


        let insert_option = assembler.symbol_table.insert(label_token.value.to_string(), label_num_value);

        // there is an error 
        // if we end of with some of something
        // because that means the line exists 
        if let Some(_) = insert_option
        {
            return Err(Assembler::create_error("Label is already defined", &label_token, vec![]));
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
        let instruction_token = Assembler::unwrap_token_option(assembler.lexical_iterator.next(), &mut assembler.lexical_iterator)?.clone();
        let instruction_option = assembler.instruction_table.get(&instruction_token.value.to_lowercase());
        let instruction_data_struct:Instruction;
        // unwrap the instruction_option
        match instruction_option
        {
            None=>{return Err(Assembler::create_error("Instruction has not been implemented yet!", &instruction_token, vec![]))},
            Some(t)=>{instruction_data_struct = t.clone()},
        }

        // gotten_tokens holds the gotten tokens
        let mut gotten_tokens:Vec<Token> = vec![];

        // holds a ref to the bestmatching grammar
        let mut best_matching_grammar:&(u8,Vec<TokenType>) = &instruction_data_struct.opcode_grammer[0];
 
        // holds a count of the number of elements that match
        let mut best_match_count: i32 = 0;


        
        // there was an error because we weren't supposed to make it this far
        return Err(Assembler::create_error("Syntax error", &gotten_tokens[gotten_tokens.len()-1], vec![best_matching_grammar.1[(best_match_count) as usize]]));
         

    }




    // expression_parser
    // converts a label expression into a single 
    // expression unless we don't wanna check variable existence in
    // that case it just returns a 
    fn expression_parser(assembler: &mut Assembler, check_variable_exitence:bool) -> Result<InsertableNum,GeneralError>
    {
        let mut operand_stack:Vec<InsertableNum> = vec![];
        let mut operator_stack:Vec<Token> = vec![];

        let mut left_parenth_count = 0;

        let mut last_gotten_token = Token::empty_token();

        loop 
        {

            let peeked_token = Assembler::unwrap_token_option(assembler.lexical_iterator.peek(0), &mut assembler.lexical_iterator)?;
            let next_peeked_token = Assembler::unwrap_token_option(assembler.lexical_iterator.peek(1), &mut assembler.lexical_iterator)?;

            // check the syntax 
            Assembler::check_expression_syntax(&last_gotten_token, &peeked_token)?;
            last_gotten_token = peeked_token.clone();

            

            // ERROR there is an unmatched left parenthese
            if (peeked_token.token_type == TokenType::Comma || peeked_token.token_type == TokenType::EOL) && left_parenth_count>0 
            {
                return Err(Assembler::create_error("Syntax error, unmatched parentheses", &peeked_token, vec![TokenType::RightParenth]))
            }
            // END 
            else if peeked_token.token_type == TokenType::RightParenth && left_parenth_count == 0 && operand_stack.len() ==1
            {
                break;
            }
            // END
            else if (peeked_token.token_type == TokenType::Comma || peeked_token.token_type == TokenType::EOL) && operand_stack.len() ==1
            {
                break;
            }
            // NUMBER its a number, insert it into the operand stack
            else if peeked_token.token_type == TokenType::Num1Bytes || peeked_token.token_type == TokenType::Num2Bytes || peeked_token.token_type == TokenType::Label
            {
                // take the peeked token 
                assembler.lexical_iterator.next();

                // insert the peeked token 
                operand_stack.push(Assembler::token_to_num(assembler, peeked_token, check_variable_exitence)?);
            }
            // OPERAND its a operator, insert it into the operator stack
            else if peeked_token.token_type == TokenType::PLUS || peeked_token.token_type == TokenType::MINUS ||peeked_token.token_type == TokenType::TIMES || peeked_token.token_type == TokenType::DIVIDE
            {   
                // take the peeked token 
                assembler.lexical_iterator.next();

                // insert the peeked token 
                operator_stack.push(peeked_token);
            }
            // OPERAND its a left parentheses
            else if peeked_token.token_type == TokenType::LeftParenth
            {
                // take the peeked token 
                assembler.lexical_iterator.next();

                // insert the peeked token 
                operator_stack.push(peeked_token);

                // add to the parenth count

                left_parenth_count = left_parenth_count +1;
            }
            // DO stuff between parentheses
            // if it is a parenthese then a comma, it is probably a sta (label+2), X kina instruction
            else if peeked_token.token_type == TokenType::RightParenth && next_peeked_token.token_type != TokenType::Comma
            {
                // get left parenth index
                let left_parenth_index_option = Assembler::next_left_parenth_index_top(&mut operator_stack);
                let left_parenth_index;
                match left_parenth_index_option
                {
                    None => return Err(Assembler::create_error("Syntax error, unexpected right parenth", &peeked_token, vec![])),
                    Some(t)=> left_parenth_index=t
                }

                // consume the right parenth 
                assembler.lexical_iterator.next();
                
                Assembler::stack_math(&mut operand_stack, &mut operator_stack, left_parenth_index)?;

                left_parenth_count = left_parenth_count-1;
            }
            // Do stuff between the start and end
            else if peeked_token.token_type == TokenType::Comma || peeked_token.token_type == TokenType::EOL || (peeked_token.token_type == TokenType::RightParenth && next_peeked_token.token_type == TokenType::Comma)
            {
                // if there is still a left parenth this
                // means there is umatched parentheses
                if left_parenth_count > 0
                {
                    return Err(Assembler::create_error("Syntax error, unmatched left parenth", &peeked_token, vec![]));
                }


                Assembler::stack_math(&mut operand_stack, &mut operator_stack, 0)?;
            }
            // if it is anything else then what is expected throw a friggen error
            else
            {
                return Err(Assembler::create_error("Syntax error, unexpected token in expression", &peeked_token, vec![TokenType::Num1Bytes, TokenType::Num2Bytes, TokenType::Label, TokenType::MINUS, TokenType::PLUS, TokenType::TIMES, TokenType::MINUS, TokenType::LeftParenth, TokenType::RightParenth]))
            }




        }
        

        Ok(operand_stack[0])
    }


    // check_expression_syntax
    // makes sure an operator is followed by an operand or left parentheses 
    // and an operand is followed by an operand 
    fn check_expression_syntax(last_token: &Token, next_token: &Token)-> Result<(),GeneralError>
    {

        // empty => "(" || "byte" || "twobyte" || "label"
        if last_token.token_type == TokenType::Empty && !(next_token.token_type == TokenType::LeftParenth ||  next_token.token_type == TokenType::Num1Bytes || next_token.token_type == TokenType::Num2Bytes || next_token.token_type == TokenType::Label)
        {
            return Err(Assembler::create_error("Syntax error, unexpected token", next_token, vec![TokenType::LeftParenth, TokenType::Num2Bytes, TokenType::Num1Bytes, TokenType::Label]));
        }
        // "(" => "(" || "byte" || "twobyte" || "label"
        else if last_token.token_type == TokenType::LeftParenth && !(next_token.token_type == TokenType::LeftParenth ||  next_token.token_type == TokenType::Num1Bytes || next_token.token_type == TokenType::Num2Bytes || next_token.token_type == TokenType::Label)
        {
            return Err(Assembler::create_error("Syntax error, unexpected token", next_token, vec![TokenType::Num1Bytes, TokenType::Num2Bytes, TokenType::Label]));
        }
        // ("byte" || "twobyte" || "label" || ")") => "+" || "-" || "/" || "*" || ")" || "EOL" || ","
        else if (last_token.token_type == TokenType::Label || last_token.token_type == TokenType::Num1Bytes || last_token.token_type == TokenType::Num2Bytes || last_token.token_type==TokenType::RightParenth) && !(next_token.token_type == TokenType::DIVIDE || next_token.token_type == TokenType::TIMES || next_token.token_type == TokenType::PLUS || next_token.token_type == TokenType::MINUS || next_token.token_type == TokenType::RightParenth || next_token.token_type == TokenType::EOL || next_token.token_type == TokenType::Comma) 
        {
            return Err(Assembler::create_error("Syntax error, unexpected token", next_token, vec![TokenType::PLUS, TokenType::MINUS, TokenType::DIVIDE, TokenType::TIMES, TokenType::RightParenth]));
        }
        // ("+" || "-" || "/" || "*") => ("label" || "byte" || "twobyte" || "(")
        else if (last_token.token_type == TokenType::PLUS || last_token.token_type == TokenType::MINUS || last_token.token_type == TokenType::DIVIDE || last_token.token_type == TokenType::TIMES) && !(next_token.token_type == TokenType::Label || next_token.token_type == TokenType::Num1Bytes || next_token.token_type == TokenType::Num2Bytes || next_token.token_type == TokenType::LeftParenth)
        {
            return Err(Assembler::create_error("Syntax error, unexpected token", next_token, vec![TokenType::Label, TokenType::Num1Bytes, TokenType::Num2Bytes]));
        }
        Ok(())
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
    fn write_token_to_file(file:&mut File, token: Token, symbol_table: &mut HashMap<String, InsertableNum>,) -> Result<(), GeneralError>
    {   
        let mut _result = Ok(0);
        match token.token_type
        {
            TokenType::Num1Bytes => 
            {
                _result = file.write(&[Assembler::one_byte_num_string_to_int(token.value)]);
            },
            TokenType::Num2Bytes =>
            {
                // convert it to a two byte number
                let two_byte_num = Assembler::two_byte_num_string_to_int(token.value);

                // get the upper and lower bytes
                let lower_byte:u8 = two_byte_num as u8;
                let upper_byte:u8 = (two_byte_num >> 8) as u8;

                // since it is little endian we store the lower byte first
                _result = file.write(&[lower_byte, upper_byte]);
            },
            TokenType::Label =>
            {
                // make sure the label exists
                let insertable_num_option = symbol_table.get(&token.value);
                let insertable_num;
                match insertable_num_option
                {
                    None => {
                        return Err(Assembler::create_error("Label doesn't exist", &token, vec![]))
                    }, 
                    Some(t) => insertable_num = *t,
                }

                // write it to file however so
                match insertable_num 
                {
                    InsertableNum::Byte(num) => _result = file.write(&[num]),
                    InsertableNum::TwoByte(num)  => 
                    {
                    // get the upper and lower bytes
                    let lower_byte:u8 = num as u8;
                    let upper_byte:u8 = (num >> 8) as u8;
                    // since it is little endian we store the lower byte first
                    _result = file.write(&[lower_byte, upper_byte]);
                    }
                }

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
                _result = file.write(&[character as u8]);
                
            },
            TokenType::String =>
            {   
                // get the string and remove the quotations ""
                let mut characters = token.value.chars();
                characters.next();
                characters.next_back();

                // write the string bytes 
                _result = file.write(&characters.as_str().as_bytes());
            },
            _ => { 
            }
        }



        match _result {

            Err(err)=> {
                let error_string = format!("Problem writing to file. details: {:?}", err);
                return Err(Assembler::create_empty_error(&error_string));
            }

            _=> Ok(())
        }


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
    // string to a u16
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


    // token_to_num
    // returns an InsertableNum 
    // given a token
    // if it can't turn it into a num it returns 0 
    // if it is a label and we want it to look up labels it returns 
    // the labels value
    fn token_to_num(assembler: &mut Assembler, token: Token, look_up_labels:bool) -> Result<InsertableNum,GeneralError>
    {
        match token.token_type
        {
            TokenType::Num1Bytes =>
            {
                return Ok(InsertableNum::Byte(Assembler::one_byte_num_string_to_int(token.value.to_string())));
            },
            TokenType::Num2Bytes =>
            {
                return Ok(InsertableNum::TwoByte(Assembler::two_byte_num_string_to_int(token.value.to_string())));
            },
            TokenType::Label => 
            {
                if look_up_labels 
                {
                    let value_option = assembler.symbol_table.get(&token.value);

                    if let None = value_option 
                    {
                        return Err(Assembler::create_error("Label not previously defined", &token, vec![]));
                    }
                    else 
                    {
                        return Ok(value_option.unwrap().clone());
                    }
                }
                else
                {
                    Ok(InsertableNum::Byte(0))
                }
            },
            _ => {
                return Ok(InsertableNum::Byte(0));
            }
        }
    }


    // do_operation
    // takes the two operands 
    // and does the given operation
    fn do_operation(operand1:InsertableNum, operand2:InsertableNum, operator:Token)-> Result<InsertableNum, GeneralError>
    {
        match operator.token_type
        {
            TokenType::PLUS =>
            {
                // return a u16
                if operand1.is_two_bytes()|| operand2.is_two_bytes()
                {
                    return Ok(InsertableNum::TwoByte(operand1.unwrap_twobyte() + operand2.unwrap_twobyte()))
                }      
                else
                {
                    return Ok(InsertableNum::Byte((operand1.unwrap_byte() + operand2.unwrap_byte()) as u8))
                }          
            },
            TokenType::MINUS =>
            {
                // return a u16
                if operand1.is_two_bytes()|| operand2.is_two_bytes()
                {
                    let i =  Wrapping(operand1.unwrap_twobyte()) - Wrapping(operand2.unwrap_twobyte());
                    return Ok(InsertableNum::TwoByte(i.0))
                }      
                else
                {
                    let i = Wrapping(operand1.unwrap_byte()) - Wrapping(operand2.unwrap_byte());
                    return Ok(InsertableNum::Byte(i.0))
                }          
            },
            TokenType::TIMES =>
            {
                // return a u16
                if operand1.is_two_bytes()|| operand2.is_two_bytes()
                {
                    let i =  Wrapping(operand1.unwrap_twobyte()) * Wrapping(operand2.unwrap_twobyte());
                    return Ok(InsertableNum::TwoByte(i.0))
                }      
                else
                {
                    let i = Wrapping(operand1.unwrap_byte()) * Wrapping(operand2.unwrap_byte());
                    return Ok(InsertableNum::Byte(i.0))
                }          
            }, 
            TokenType::DIVIDE =>
            {

                // cannot divide by zero
                if operand2.unwrap() == 0
                {
                    return Err(Assembler::create_error("Cannot divide by zero", &operator, vec![]));
                }


                // return a u16
                if operand1.is_two_bytes()|| operand2.is_two_bytes()
                {
                    let i =  Wrapping(operand1.unwrap_twobyte()) / Wrapping(operand2.unwrap_twobyte());
                    return Ok(InsertableNum::TwoByte(i.0));
                }      
                else
                {
                    let i = Wrapping(operand1.unwrap_byte()) /  Wrapping(operand2.unwrap_byte());
                    return Ok(InsertableNum::Byte(i.0))
                }          
            }
            _ => {Ok(InsertableNum::Byte(0))}
        }
    }



    // next_left_parenth_index_top
    // returns the next left parenth
    // in a vector stack from the 
    // the top of the vector
    fn next_left_parenth_index_top(s:&mut Vec<Token>) -> Option<usize> 
    {

        let mut t_index:i32 = s.len() as i32-1;
        for t in s.iter().rev(){
            if t.token_type == TokenType::LeftParenth
            {
               return Some(t_index as usize);
            }
            t_index = t_index-1;
        }

        None
    }



    // stack_math
    // does math between an operand stack and an operator stack
    // but only does it from a given index in the operator stack 
    // pops the left parenth off if the index given is one
    fn stack_math(operand_stack:&mut Vec<InsertableNum>, operator_stack:&mut Vec<Token>, index: usize) -> Result<(),GeneralError> 
    {

        if operand_stack.len()>index && operator_stack[index].token_type == TokenType::LeftParenth  
        {
            operator_stack.remove(index);
        }
        
        
        if operand_stack.len() == 1
        {
            return Ok(())
        }
        // else if there isn't enough operands
        else if !(operand_stack.len() > (operator_stack.len() - index))
        {
            return Err(Assembler::create_error("Syntax error, not enoug operands for operator", &operator_stack[0], vec![]))
        }

        // do all the times and divide operations up the stack 
        let mut copy_index = index.clone();
        while copy_index != operator_stack.len() 
        {
            
            let operator = &operator_stack[copy_index];

            let operand_1_index = operand_stack.len() as i32 - (operator_stack.len() as i32 - copy_index as i32+1);


            let operand_1 = &operand_stack[operand_1_index as usize];
            let operand_2 = &operand_stack[operand_1_index as usize+1];


            if operator.token_type == TokenType::TIMES || operator.token_type == TokenType::DIVIDE
            {
                // replace the number on the stack
                let replaced_index = operand_1_index;

                operand_stack[replaced_index as usize] = Assembler::do_operation(*operand_1, *operand_2, operator.clone())?;

                // remove the next position 
                operand_stack.remove(replaced_index as usize+1);

                operator_stack.remove(copy_index as usize);
            }
            else 
            {
                copy_index = copy_index+1;
            }
        }

         // do all the add and subtract operations up the stack 
         let mut copy_index = index.clone();
         while copy_index != operator_stack.len() 
         {
            let operator = &operator_stack[copy_index];

            let operand_1_index = operand_stack.len() as i32 - (operator_stack.len()as i32 - copy_index as i32+1);

            let operand_1 = &operand_stack[operand_1_index as usize];
            let operand_2 = &operand_stack[operand_1_index as usize+1];



            if operator.token_type == TokenType::PLUS || operator.token_type == TokenType::MINUS
            {
                // replace the number on the stack
                let replaced_index = operand_1_index;
                
                operand_stack[replaced_index as usize] = Assembler::do_operation(*operand_1, *operand_2, operator.clone())?;

                // remove the next position 
                operand_stack.remove(replaced_index as usize+1);

                operator_stack.remove(copy_index as usize);
            }
            else 
            {
                copy_index = copy_index+1;
            }
         }



        Ok(())
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
                Assembler::write_token_to_file(&mut assembler.file_writer, token, &mut assembler.symbol_table)?;
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

