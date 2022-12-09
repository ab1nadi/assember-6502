mod instruction;

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
    fn first_pass(&mut self) ->Result<(),GeneralError>
    {
        // the iterator that will be used
        let mut iter = PeekWrapper::new(self.lexical_analyzer.get_iterator(), 3);

        let mut addedBytes = 0;
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
                TokenType::Instruction => 
                {
                    addedBytes = Assembler::instruction_parser( &self.instruction_table,&mut iter)?;
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

    // instruction_parser_first_pass
    // essentially this parses an instruction
    // from the lexical analyzer
    fn instruction_parser(instruction_table: &HashMap<String,Instruction>, iterator: &mut PeekWrapper<LexicalIterator>)-> Result<u32,GeneralError>
    {
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


            // iterate over all the possible tokens in a grammar
            for (i, token_type_grammar) in grammar_vec.1.iter().enumerate()
            {


                // only get tokens when we need to
                if (gotten_tokens.len() as i32)-1 < i as i32
                {
                    gotten_tokens.push(Assembler::unwrap_token_option(iterator.next(),iterator)?);
                }


                // allow a type coercion from label to 2bytes num because, ultimately,  thats what labels are
                if gotten_tokens[i].token_type == TokenType::Label && *token_type_grammar == TokenType::Num2Bytes
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
                    match_count = match_count +1;
                }
            }   


            // if we matched totally this is what we want it to be 
            if(matched)
            {
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
            Ok(32)
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

}

