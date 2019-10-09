use clap::{clap_app, crate_version, AppSettings::ArgRequiredElseHelp, ArgMatches};
use std::fs::read_to_string;
use xasm::compile::Compile;
use xassembler::{Golang, Rust};

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
            (@arg PACKAGES: -p --packages +takes_value ... "Paths to foreign packages to use")
        )
        (@subcommand compile =>
            (version: "0.1")
            (about: "Compile a xasm file")
            (author: "Adam McDaniel <adam.mcdaniel17@gmail.com>")
            (@arg INPUT: +required "Input file")
            (@arg OUTPUT: -o --output +takes_value default_value("a.out") "Output path to executable")
            (@arg PACKAGES: -p --packages +takes_value ... "Paths to foreign packages to use")
        )
    )
    .setting(ArgRequiredElseHelp)
    .get_matches();

    if matches.is_present("golang") {
        cli::<Golang>(matches)?
    } else if matches.is_present("rust") {
        cli::<Rust>(matches)?
    }

    Ok(())
}

fn cli<T: Compile>(matches: ArgMatches) -> Result<(), String> {
    if let Some(matches) = matches.subcommand_matches("run") {
        let input = matches.value_of("INPUT").unwrap();

        let packages = match matches.values_of("PACKAGES") {
            Some(list) => list.map(|s| s.trim_matches('/')).collect::<Vec<&str>>(),
            None => vec![]
        };

        if let Ok(contents) = read_to_string(input) {
            match T::assemble(&contents) {
                Ok(compiled) => T::run_subcommand(&compiled, packages)?,
                Err(e) => println!("{}", e),
            }
        } else {
            eprintln!("Could not open file {}", input);
        }
    } else if let Some(matches) = matches.subcommand_matches("compile") {
        let input = matches.value_of("INPUT").unwrap();
        let output = matches.value_of("OUTPUT").unwrap();

        let packages = match matches.values_of("PACKAGES") {
            Some(list) => list.map(|s| s.trim_matches('/')).collect::<Vec<&str>>(),
            None => vec![]
        };
        
        if let Ok(contents) = read_to_string(input) {
            match T::assemble(&contents) {
                Ok(compiled) => T::compile_subcommand(&compiled, packages, &output)?,
                Err(e) => println!("{}", e),
            }
        } else {
            eprintln!("Could not open file {}", input);
        }
    }

    Ok(())
}
