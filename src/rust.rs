use std::fs::{rename, write};
use dunce::canonicalize;
use std::process::{Command, Stdio};
use xassembler::Rust;

use crate::compile::Compile;

pub const MANIFEST: &str = r#"
[package]
name = "target"
version = "0.1.0"
authors = ["adam-mcdaniel <adam.mcdanie17@gmail.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
xmachine = "0.2.1"
"#;


fn make_absolute(s: String) -> String {
    // Replace backslashes with doubles to make a valid string
    canonicalize(s).unwrap().display().to_string().replace("\\", "\\\\")
}

impl Compile for Rust {
    fn compile_subcommand(compiled: &str, dependency_paths: Vec<&str>, output_path: &str) -> Result<(), String> {
        Self::build(compiled, dependency_paths)?;

        match (rename(Self::build_dir()? + "/target/release/target", output_path),
                rename(Self::build_dir()? + "/target/release/target.exe", output_path),
                rename(Self::build_dir()? + "/target/release/target.bin", output_path)) {
            (Err(_), Err(_), Err(_)) => {
                return Err(format!("Could not move compiled executable to `{}`. Your program most likely did not compile successfully. Check your foreign functions' names", output_path))
            },
            _ => {}
        };

        Ok(())
    }

    fn run_subcommand(compiled: &str, dependency_paths: Vec<&str>) -> Result<(), String> {
        Self::build(compiled, dependency_paths)?;

        if let Err(_) = Command::new("cargo")
            .args(&["run", "--release"])
            .current_dir(Self::build_dir()?)
            .stdout(Stdio::inherit())
            .stdin(Stdio::inherit())
            .output() {
			return Err(String::from("Could not run `cargo`, is rust properly installed?"))
		}

        Ok(())
    }

    fn build(compiled: &str, dependency_paths: Vec<&str>) -> Result<(), String> {
        let result = format!("{}\n{}", dependency_paths
                                        .iter()
                                        .map(|s|
                                            format!("extern crate {lib};\nuse {lib}::*;",
                                                lib=s.rsplit("/").collect::<Vec<&str>>()[0])
                                            ).collect::<Vec<String>>()
                                        .join("\n"), compiled);

        if let Err(_) = Command::new("cargo")
            .current_dir(Self::home_dir()?)
            .args(&["new", Self::BUILD_DIR_NAME])
            .output() {
			return Err(String::from("Could not run `cargo`, is rust properly installed?"))
        }

        if let Err(_) = write(Self::build_dir()? + "/src/main.rs", result) {
			return Err(String::from("Could not write compiled output to output crate"))
        }

        
        if let Err(_) = write(Self::build_dir()? + "/Cargo.toml", format!("{}\n{}", MANIFEST,
                                    dependency_paths
                                        .iter()
                                        .map(|s| {
                                            let path = make_absolute(s.to_string());
                                            format!("{package} = {{path=\"{path}\"}}",
                                                package=std::path::Path::new(&path).file_stem().unwrap().to_str().unwrap(),
                                                path=path)
                                        })
                                        .collect::<Vec<String>>()
                                        .join("\n"))) {
			return Err(String::from("Could not write to Cargo.toml in output crate"))
        }

        if let Err(_) = Command::new("cargo")
            .args(&["build", "--release"])
            .current_dir(Self::build_dir()?)
            .output() {
			return Err(String::from("Could not run `cargo`, is rust properly installed?"))
		}

        Ok(())
    }

    const TERMINATE: &'static str = "\n}";
    const BUILD_DIR_NAME: &'static str = "xasm_build";
    const PRELUDE: &'static str = r#"
extern crate xmachine;
use xmachine::{Machine, Value};


pub fn xasm_dict(xasm: &mut Machine) {
    xasm.push(Value::tree());
}


fn main() {
    let mut xasm = Machine::new();
    xasm.push(Value::function(xasm_dict, &xasm));
    xasm.push(Value::string("dict"));
    xasm.store();
"#;
}
