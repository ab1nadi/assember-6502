mod error;

use fancy_regex::Regex;
use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::lexical_analyzer::error::LexicalError;
use crate::unwrap_result_or_return_err;






// the LexicalAnalyzer
// public struct 
#[derive(Debug)]
pub struct LexicalAnalyzer
{
    file_name:  String,
    reader: BufReader<File>,
    current_line: String, 
    return_eof: bool,                   // the difference between these is weather eof has or hasnt been returned yet
    returned_eof: bool,                 //
    return_eol: bool,
    token_parsers: Vec<TokenParser>, 
    remove_comments: bool,
    file_line: u32,
    logical_line: u32,
    current_line_new: bool,
}

// token
// a struct that
// holds a lexical token 
#[derive(Debug)]
pub struct Token
{
    pub token_type: TokenType,
    pub value: String,

    logical_line: u32,      // logical line doesn't include newlines or comment lines
    file_line: u32,         // file line includes newlines and comment lines 
}

// TokenType
// enumarates the types of tokens
// needs all this derive crap to be used and compared against
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
pub enum TokenType 
{
    Instruction,
    Register,
    HexNum2Bytes,
    HexNum4Bytes,
    DecNum2Bytes,
    DecNum4Bytes,
    Character,
    Hash,          // tells us it is immidiete addressing 
    Comment,           
    LeftParenth,
    RightParenth,
    Comma,
    Collon,
    Label, 
    Directive, 
    Garbage,    // the catchall for 
    EOF,
    EOL,        // end of line 
}

// TokenParser
// struct that holds a regular expression string
// and the type of token that regular expresssion 
// wants to find 
#[derive(Debug)] 
pub struct TokenParser
{
    reg: String,
    token_type: TokenType,
}


// the implementaion for
// the lexical analyzer
impl LexicalAnalyzer
{
    // exposed api /////////////////////////////////////////////
    ////////////////////////////////////////////////////////////
    
    // new 
    // returns a new lexical 
    // analyzer
    pub fn new(file_name:String, remove_comm: bool) -> Result<LexicalAnalyzer, LexicalError>
    {
        let file = unwrap_result_or_return_err!(File::open(&file_name), "problem opening file");
        
        Ok(LexicalAnalyzer 
        {
            file_name: file_name,
            reader: BufReader::new(file),
            current_line: "".to_string(),
            return_eof: false,
            returned_eof: false,
            return_eol: false,
            token_parsers: LexicalAnalyzer::get_token_parsers(),
            remove_comments: remove_comm,
            logical_line:0,
            file_line:0,
            current_line_new: true
        })
    }

    // reset
    // essentially this resets
    // the iterator to the beginning 
    // of the file
    pub fn reset(& mut self)-> Result<(), LexicalError>
    {
        // reopen the file
        let file = unwrap_result_or_return_err!(File::open(&self.file_name), "problem opening file");
        self.reader = BufReader::new(file);

        // reset eof 
        self.return_eof = false;
        self.current_line_new =true;
        self.file_line = 0;
        self.logical_line =0;

        // return ok 
        Ok(())
    }

    // exposed api end //////////////////////////////////////////
    ////////////////////////////////////////////////////////////

    // get_line
    // private function that
    // getsline 
    fn get_line(& mut self) -> Result<(), LexicalError>
    {



        // if the current line is empty
        // keep getting new lines 
        while self.current_line == ""
        {
            // whatever we return is a new line
            self.current_line_new = true;
            // we have gotten a new file line 
            self.file_line +=1;
            // read from the file reader a line 
            match self.reader.read_line(&mut self.current_line) {

                // something bad happened 
                Err(err) => {
                    return Err(LexicalError::new(&err.to_string()));
                },
                // eof
                // just return 
                Ok(0) => {
                    self.return_eof = true;
                    return Ok(());
                }
                // there is actual data 
                _ => {
                    // trim it 
                    // this will remove newlines and make it an empty string if there is nothing there 
                    self.current_line = self.current_line.trim().to_string();
                }
            }

        }

        Ok(())
    }


    // parse_next_token
    // using regular expressions
    // this bad boy creates a token
    // removes it from the current line
    fn parse_next_token(& mut self) -> Result<Token, LexicalError>
    {
        // get a line 
        // if we dont already have one
        let result = self.get_line();
        match result{
            Ok(_) => {},
            Err(err) => return Err(err),
        }



        let op = self.return_eol_eof_if();
        if let Some(returnable) = op
        {
            return returnable;
        }


        for p in &self.token_parsers 
        {
                   // create the regx
        let reg = Regex::new(p.reg.as_str()).unwrap();
        let found_option = reg.find(&self.current_line).unwrap();
        

        
        match found_option 
        {
            
            // a token was found
            Some(caps) =>
            {

                // need to do to string and clone because it barrows a slice from self.current line if we don't 
                // shouldn't be that slow, later mabye I'll figure something else out 
                let captured_text = caps.as_str().trim().to_string().clone();

                // remove that item from the current line 
                self.current_line = self.current_line.replace(&captured_text, "");

                
                if self.current_line == ""
                {
                    // only return it if this isn't a comment line
                    // and this is a current newline 

                    if !(self.current_line_new && p.token_type == TokenType::Comment && self.remove_comments)
                     {   
                        self.return_eol = true;
                     }
              
                }


                // logical line addition
                // basically the line count for wahtever we only care about
                if self.current_line_new 
                {
                    self.current_line_new = false;
                    self.logical_line+= 1;
                }


                // return an eol if the line is empty



                if self.remove_comments && p.token_type == TokenType::Comment
                {
                    return self.parse_next_token();
                }

            
                
                return Ok(Token{token_type:p.token_type, value: captured_text.to_string(), logical_line: self.logical_line, file_line: self.file_line,});
            },  

            // do nothing if it didn't find anying 
            _ => {},
        }
 
        }

        // this should never happen, because there is a garbage token
        // that collects everything left, but in the case I messed up that
        // regular expression this will catch 
        Err(LexicalError { details: ("No token selected".to_string()) })
    }   

    // get_token_parsers
    // returns a list of regular expression
    // strings and their token type that parse
    // tokens
    fn get_token_parsers() -> Vec<TokenParser>
    {
        vec![
            TokenParser{reg: r"^\\\\[\w\W]*".to_string(),
            token_type:TokenType::Comment},
            TokenParser{reg:r"^(?i)(adc|and|asl|bcc|bcs|beq|bit|bmi|bne|bpl|brk|bvc|bvs|clc|cld|cli|clv|cmp|cpx|cpy|dec|dex|dey|eor|inc|inx|iny|jmp|jsr|lda|ldx|ldy|lsr|nop|ora|pha|php|pla|plp|rol|ror|rti|rts|sbc|sec|sed|sei|sta|stx|sty|tax|tay|tsx|txa|txs|tya)((?=\W)|(?=\s)|\z)".to_string(), 
                        token_type:TokenType::Instruction},
            TokenParser{reg: r"^\.[a-zA-Z]+((?=\W)|(?=\s)|\z)".to_string(),
                        token_type:TokenType::Directive},
            TokenParser{reg: r"^\(".to_string(),
                        token_type:TokenType::LeftParenth},
            TokenParser{reg: r"^\)".to_string(), 
                        token_type:TokenType::RightParenth},
            TokenParser{reg: r"^#".to_string(),
                        token_type:TokenType::Hash},
            TokenParser{reg:r"^\'[a-zA-Z0-9]\'".to_string(),
                        token_type:TokenType::Character},
            TokenParser{reg:r"^\:".to_string(),
                        token_type:TokenType::Collon},
            TokenParser{reg:r"^(a|A|x|X|y|Y)+((?=\W)|(?=\s)|\z|,)".to_string(),
                        token_type:TokenType::Register},
            TokenParser{reg:r"^\,".to_string(),
                        token_type:TokenType::Comma},
            TokenParser{reg:r"^\$([0-9A-Fa-f][0-9A-Fa-f]|[0-9A-Fa-f])((?=\W)|(?=\s)|\z|,)".to_string(),
                        token_type:TokenType::HexNum2Bytes},
            TokenParser{reg:r"^\$[0-9A-Fa-f]+((?=\W)|(?=\s)|\z|,)".to_string(),
                        token_type:TokenType::HexNum4Bytes},
            TokenParser{reg:r"^^(([0-2][0-5][0-5])|([0-1][0-9][0-9])|([0-9][0-9]))((?=\W)|(?=\s)|\z|,)".to_string(),
                        token_type:TokenType::DecNum2Bytes},
            TokenParser{reg:r"^[0-9]+((?=\W)|(?=\s)|\z|,)".to_string(),
                        token_type:TokenType::DecNum4Bytes},
            TokenParser{reg:r"^([0-9A-za-z$#@!?*&^%~\.;\[\]])+((?=\W)|(?=\s)|\z|,)".to_string(),
                        token_type:TokenType::Label},
            TokenParser{reg:r"^[\w\W]+".to_string(),
                        token_type:TokenType::Garbage}  
        ]
    }


    // return_eol_or_eof_if
    // returns the eol if we are at an
    // eol
    // returns the eof if we are at the eof
    fn return_eol_eof_if(& mut self) -> Option<Result<Token, LexicalError>>
    {

        if self.return_eol
        {
            self.return_eol = false;
            return Some(Ok(Token{
                token_type:TokenType::EOL,
                value: "".to_string(),
                logical_line: self.logical_line,
                file_line: self.file_line,
            }));
        }

        if self.return_eof
        {
            self.returned_eof = true;
            return Some(Ok(Token{
                token_type:TokenType::EOF,
                value: "".to_string(),
                logical_line: self.logical_line,
                file_line: self.file_line,
            }));
        }

        None
    }
    

}




// implementing the Iterator trait
// for the Lexical Analyzer
// so that we can iterate over tokens
// infact I expect this to be the only 
// to interact with tokens
impl Iterator  for  LexicalAnalyzer
{

    type Item= Result<Token, LexicalError>;

    fn next(&mut self) -> Option<Self::Item>
    {   

        // so that the iterator
        // acutally ends
        // we want to return none after eof 
        if self.returned_eof
        {
            return None;
        }


        // parse the next token 
        let returned = self.parse_next_token();

        // trim whitespaces from the current line so next token char is first
        // in the current line
        self.current_line = self.current_line.trim().to_string();

        // return the Token
        Some(returned)
    }
}
