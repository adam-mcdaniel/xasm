use xasm::{
    compile::{
        compile_subcommand,
        run_subcommand,
        compiler_output
    }
};

use clap::{
    clap_app, crate_version,
    AppSettings::ArgRequiredElseHelp
};

use std::fs::read_to_string;

fn main() -> Result<(), String> {
    let matches = clap_app!(xasm =>
        (version: crate_version!())
        (author: "Adam McDaniel <adam.mcdaniel17@gmail.com>")
        (about: "Compiler for the xasm programming language")
        (@subcommand run =>
            (version: "0.1")
            (about: "Run a xasm file")
            (author: "Adam McDaniel <adam.mcdaniel17@gmail.com>")
            (@arg INPUT: +required "Input file")
        )
        (@subcommand compile =>
            (version: "0.1")
            (about: "Compile a xasm file")
            (author: "Adam McDaniel <adam.mcdaniel17@gmail.com>")
            (@arg INPUT: +required "Input file")
            (@arg OUTPUT: -o --output +takes_value +required "Output path to executable")
        )
    )
    .setting(ArgRequiredElseHelp)
    .get_matches();
    

    if let Some(matches) = matches.subcommand_matches("run") {
        let input = matches.value_of("INPUT").unwrap();
        
        if let Ok(contents) = read_to_string(input) {
            match compiler_output(&contents) {
                Ok(compiled) => run_subcommand(&compiled)?,
                Err(e) => println!("{}", e)
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("compile") {
        let input = matches.value_of("INPUT").unwrap();
        let output = matches.value_of("OUTPUT").unwrap();
        
        if let Ok(contents) = read_to_string(input) {
            match compiler_output(&contents) {
                Ok(compiled) => compile_subcommand(&compiled, &output)?,
                Err(e) => println!("{}", e)
            }
        }
    }

    Ok(())
}
