
mod assembler;
use crate::assembler::Assembler;
use wasm_bindgen::prelude::*;
use js_sys;
extern crate web_sys;


// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// the struct the run
// function returns
// so that we can tell if it 
// ran into an error or not
// and extract the data string
#[wasm_bindgen]
pub struct Data {
    data: Vec<u8>,
    error_value: String,
    error: bool,
}

#[wasm_bindgen]
impl Data 
{
    pub fn new() -> Data 
    {
        Data {data: vec![], error: false, error_value: String::new()}
    }

    pub fn data(&self) -> js_sys::Uint8Array {
        js_sys::Uint8Array::from(self.data.as_slice())         // just copies it but whatevs, I'll figure it out later so it doesn't
    }

    pub fn error_value(&self) -> String
    {
        self.error_value.to_string()
    }

    pub fn error(&self) -> bool {
        self.error
    }
}



// The function that will be exposed to javasript
// for it to run the assembler
#[wasm_bindgen]
pub fn run(assembly_text: &str)->Data {


    let mut ass_text = assembly_text.to_string();
    let ass_result = Assembler::new(&mut ass_text);
    let mut ass;



    // what will be returned in a json format
    let mut d: Data = Data::new();

    // unwrap the creation of the assembler
    if let Err(err) = ass_result 
    {
        d.error = true;
        d.error_value = err.to_string();
    }
    else  
    {
        ass = ass_result.unwrap();



        // unwrap the running of the assembler
        let result = ass.run();

        if let Err(err) = result 
        {
            d.error = true;
            d.error_value = err.to_string();
        }
        else 
        {
           
            d.data = ass.object_code;
        }

    }
    

   return d;

}