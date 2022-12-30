
mod assembler;
use std::env;
use crate::assembler::Assembler;
use wasm_bindgen::prelude::*;
use std::io::Read;
pub fn run_()
{
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


#[wasm_bindgen]
pub fn run(assembly_text: &str)->String {
    let ass_result = Assembler::new_from_string(assembly_text.to_string());
    let mut ass;


    // unwrap the creation of the assembler
    if let Err(err) = ass_result 
    {
        return err.to_string();
    }
    else  
    {
        ass = ass_result.unwrap();
    }


    // unwrap the running of the assembler
    let result = ass.run();

    if let Err(err) = result 
    {
        return err.to_string();
    }


    // unwrap the parsing of the object file
    let result = ass.get_obj_str();

    if let Err(err) = result
    {
        return err.to_string();
    }
    else  {
        return result.unwrap();
    }

}