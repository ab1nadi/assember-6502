
mod assembler;
use crate::assembler::Assembler;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(assembly_text: &str)->String {
    let mut ass_text = assembly_text.to_string();
    let ass_result = Assembler::new(&mut ass_text);
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
    else  
    {
        return result.unwrap();
    }

}