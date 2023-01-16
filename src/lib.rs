
mod assembler;
use crate::assembler::Assembler;
use std::os::raw::c_char;
use std::ffi::CString;
use core::ffi::CStr;

static mut STRING_POINTER: *mut c_char = 0 as *mut c_char;


#[no_mangle]
pub extern fn assemble(file_name_c:  *const c_char, out_put_c:  *const c_char) -> *mut c_char {

    let file_name;
    let out_put;
    
    unsafe {
        file_name = CStr::from_ptr(file_name_c).to_str().expect("Can not read string argument.").to_string();
        out_put = CStr::from_ptr(out_put_c).to_str().expect("Can not read string argument.").to_string();
    }

    let s = run(&file_name, &out_put);


    let pntr = CString::new(s).unwrap().into_raw();

    //store it in our static variable (REQUIRES UNSAFE)
    unsafe {
        STRING_POINTER = pntr;
    }

    //return the c_char
    return pntr;
}

#[no_mangle]
pub extern fn free_string() {
    unsafe {
        let _ = CString::from_raw(STRING_POINTER);
        STRING_POINTER = 0 as *mut c_char;
    }
}

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