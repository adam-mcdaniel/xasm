use clap::{clap_app, crate_version, AppSettings::ArgRequiredElseHelp, ArgMatches};
use xassembler::{Golang, Rust};
use std::fs::read_to_string;
use xasm::compile::Compile;

fn main() -> Result<(), String> {
    let matches = clap_app!(xasm =>
        (version: crate_version!())
        (author: "Adam McDaniel <adam.mcdaniel17@gmail.com>")
        (about: "Compiler for the xasm programming language")
        (@group target +required =>
            (@arg golang: -g --go "Compile to Golang")
            (@arg rust: -r --rs "Compile to Rust")
        )
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
            (@arg OUTPUT: -o --output +takes_value default_value("a.out") "Output path to executable")
        )
    )
    .setting(ArgRequiredElseHelp)
    .get_matches();

    if matches.is_present("golang") {
        cli::<Golang>(matches)?
    } else if matches.is_present("rust") {
        cli::<Rust>(matches)?
    } else {
        println!("no target!");
    }

    Ok(())
}


fn cli<T: Compile>(matches: ArgMatches) -> Result<(), String> {
    if let Some(matches) = matches.subcommand_matches("run") {
        let input = matches.value_of("INPUT").unwrap();

        if let Ok(contents) = read_to_string(input) {
            match T::compiler_output(&contents) {
                Ok(compiled) => T::run_subcommand(&compiled)?,
                Err(e) => println!("{}", e),
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("compile") {
        let input = matches.value_of("INPUT").unwrap();
        let output = matches.value_of("OUTPUT").unwrap();

        if let Ok(contents) = read_to_string(input) {
            match T::compiler_output(&contents) {
                Ok(compiled) => T::compile_subcommand(&compiled, &output)?,
                Err(e) => println!("{}", e),
            }
        }
    }

    Ok(())
}