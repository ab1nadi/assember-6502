
mod assembler;
use crate::assembler::Assembler;


pub fn run(file_name: &String, out_put: &String) -> String
{

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
            let returned = format!("ERROR: \n{}", err.to_string());
            returned
        },

        _ => {return "Success!".to_string()}
    }
}