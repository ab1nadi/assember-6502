
mod assembler;
use crate::assembler::Assembler;
use wasm_bindgen::prelude::*;



// the struct the run
// function returns
// so that we can tell if it 
// ran into an error or not
// and extract the data string
#[wasm_bindgen]
pub struct Data {
    data: String,
    error: bool,
}

#[wasm_bindgen]
impl Data 
{
    pub fn data(&self) -> String {
        self.data.to_string()
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
    let mut d: Data = Data {data: String::new(), error: false};

    // unwrap the creation of the assembler
    if let Err(err) = ass_result 
    {
        d.error = true;
        d.data = err.to_string();
    }
    else  
    {
        ass = ass_result.unwrap();

        // unwrap the running of the assembler
        let result = ass.run();

        if let Err(err) = result 
        {
            d.error = true;
            d.data = err.to_string();
        }
        else 
        {
            d.data = ass.object_code;
        }

    }
    

   return d;

}