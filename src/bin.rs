use asm_6502_lib::run;
use std::env;

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
    

    println!("{}", run(file_name, out_put));
}

