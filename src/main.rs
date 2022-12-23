mod assembler;
mod lexical_analyzer;
mod peek_wrapper;
mod gen_errors;
use std::env;
use crate::assembler::Assembler;

fn main() {

    let args: Vec<String> = env::args().collect();

    // if args aren't big enough return
    if args.len() < 3
    {
        println!("Expected 2 arguments: input file name and output file name");
        return;
    }

    let file_name = &args[1];
    let out_put = &args[2];
    

    let result;
    let ass_result = Assembler::new(file_name, out_put);


    if let Err(err) = ass_result 
    {
        result = Err(err);
    }
    else 
    {   
        // we know its not an error
        // so it can just be unwraped
        result = ass_result.unwrap().run();
    }

    match result 
    {
        Err(err) =>
        {
            println!("ERROR:");
            println!("{}", err.to_string());
        },

        _ => {println!("Success!")}
    }
}
