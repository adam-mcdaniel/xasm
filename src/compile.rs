use xassembler::compile;
use std::process::{Command, Stdio};
use std::fs::{write, rename};
use crate::constants::{MANIFEST, BIN_PRELUDE};

pub fn compile_subcommand(compiled: &str, output_path: &str) -> Result<(), String> {
    build(compiled)?;

    rename(
        build_dir()? + "/target/release/target",
        output_path
    ).unwrap();

    Ok(())
}

pub fn run_subcommand(compiled: &str) -> Result<(), String> {
    build(compiled)?;

    Command::new("cargo")
        .args(&["run", "--release"])
        .current_dir(build_dir()?)
        .stdout(Stdio::inherit())
        .output().unwrap();

    Ok(())
}

fn build(compiled: &str) -> Result<(), String> {
    match Command::new("cargo")
        .current_dir(home_dir()?)
        .args(&["new", BUILD_DIR_NAME])
        .output() { _ => () };
    
    write(build_dir()? + "/src/main.rs", compiled)
        .expect("Could not write to file target/src/main.rs");
        
    write(build_dir()? + "/Cargo.toml", MANIFEST)
        .expect("Could not write to target/Cargo.toml");
        
    Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir(build_dir()?)
        .output().unwrap();

    Ok(())
}



pub fn compiler_output(script: &str) -> Result<String, String> {
    Ok(format!("{} {}\n}}", BIN_PRELUDE, compile(script)?))
}


pub const BUILD_DIR_NAME: &str = "xasm_build";

pub fn home_dir() -> Result<String, String> {
    let home = dirs::home_dir().ok_or(
        String::from("No home directory in this environment")
    )?;
    Ok(home.to_str().ok_or(
        String::from("No home directory in this environment")
    )?.to_string())
}


pub fn build_dir() -> Result<String, String> {
    Ok(home_dir()? + "/" + BUILD_DIR_NAME)
}


