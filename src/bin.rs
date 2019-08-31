use xassembler::compile;
use clap::{
    clap_app, crate_version,
    AppSettings::ArgRequiredElseHelp
};

use std::process::{Command, Stdio};
use std::fs::{read_to_string, write, rename};


const BUILD_DIR_NAME: &str = "xasm_build";

fn home_dir() -> Result<String, String> {
    let home = dirs::home_dir().ok_or(
        String::from("No home directory in this environment")
    )?;
    Ok(home.to_str().ok_or(
        String::from("No home directory in this environment")
    )?.to_string())
}


fn build_dir() -> Result<String, String> {
    Ok(home_dir()? + "/" + BUILD_DIR_NAME)
}


fn main() -> Result<(), String> {
    let matches = clap_app!(xasm =>
        (version: crate_version!())
        (author: "Adam McDaniel <adam.mcdaniel17@gmail.com>")
        (about: "Compiler for the xasm programming language")
        (@arg INPUT: +required "Input file")
        (@arg OUTPUT: -o --output +takes_value "Output file for executable")
    )
    .setting(ArgRequiredElseHelp)
    .get_matches();


    if let Some(input) = matches.value_of("INPUT") {
        if let Ok(contents) = read_to_string(input) {
            match compile_bin(&contents) {
                Ok(compiled) => {

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

                    println!("Successfully compiled crate!");
                        
                    Command::new("cargo")
                        .args(&["run", "--release"])
                        .current_dir(build_dir()?)
                        .stdout(Stdio::inherit())
                        .output().unwrap();
                },
                Err(e) => println!("{}", e)
            }
        }

        if let Some(output) = matches.value_of("OUTPUT") {
            rename(build_dir()? + "/target/release/target", output).unwrap();
        }
    }
    Ok(())
}


pub fn compile_bin(script: &str) -> Result<String, String> {
    Ok(format!("{} {}\n}}", BIN_PRELUDE, compile(script)?))
}

pub const MANIFEST: &str = r#"
[package]
name = "target"
version = "0.1.0"
authors = ["adam-mcdaniel <adam.mcdanie17@gmail.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
xmachine = "0.1.2"
"#;

pub const BIN_PRELUDE: &str = r#"
extern crate xmachine;
use xmachine::{Machine, Value, Ref};


fn dict(xasm: &mut Machine) {
    xasm.push(Value::tree());
}

fn list(xasm: &mut Machine) {
    xasm.push(Value::list());
}

fn push(xasm: &mut Machine) {
    if let Some(list_value) = xasm.pop() {
        if let Value::List(mut l) = (*list_value).clone() {
            if let Some(value) = xasm.pop() {
                l.push(value);
                xasm.push(Ref::new(Value::List(l)));
            }
        }
    }
}

fn pop(xasm: &mut Machine) {
    if let Some(value) = xasm.pop() {
        if let Value::List(mut l) = (*value).clone() {
            let last_value = l[l.len() - 1].clone();
            l.pop();
            xasm.push(last_value.copy());
            xasm.push(Ref::new(Value::List(l)));
        }
    }
}

fn len(xasm: &mut Machine) {
    if let Some(value) = xasm.pop() {
        if let Value::List(l) = (*value).clone() {
            xasm.push(Value::number(l.len() as f64));
        }
    }
}

fn print(xasm: &mut Machine) {
    if let Some(string) = xasm.pop() {
        print!("{}", string);
    }
}

fn println(xasm: &mut Machine) {
    if let Some(string) = xasm.pop() {
        println!("{}", string);
    }
}

fn new(xasm: &mut Machine) {
    if let Some(class) = xasm.pop() {
        class.call(xasm);
        xasm.push(Value::string("new"));
        xasm.method_call();
    }
}


fn add(xasm: &mut Machine) {
    let first = xasm.pop();
    let second = xasm.pop();

    if let (Some(m), Some(n)) = (first, second) {
        let m_f = m.to_string().parse::<f64>().unwrap();
        let n_f = n.to_string().parse::<f64>().unwrap();

        xasm.push(
            Value::number(m_f + n_f)
        );
    }
}

fn sub(xasm: &mut Machine) {
    let first = xasm.pop();
    let second = xasm.pop();

    if let (Some(m), Some(n)) = (first, second) {
        let m_f = m.to_string().parse::<f64>().unwrap();
        let n_f = n.to_string().parse::<f64>().unwrap();

        xasm.push(
            Value::number(m_f - n_f)
        );
    }
}

fn mul(xasm: &mut Machine) {
    let first = xasm.pop();
    let second = xasm.pop();

    if let (Some(m), Some(n)) = (first, second) {
        let m_f = m.to_string().parse::<f64>().unwrap();
        let n_f = n.to_string().parse::<f64>().unwrap();

        xasm.push(
            Value::number(m_f * n_f)
        );
    }
}

fn div(xasm: &mut Machine) {
    let first = xasm.pop();
    let second = xasm.pop();

    if let (Some(m), Some(n)) = (first, second) {
        let m_f = m.to_string().parse::<f64>().unwrap();
        let n_f = n.to_string().parse::<f64>().unwrap();

        xasm.push(
            Value::number(m_f / n_f)
        );
    }
}

fn main() {
    let mut xasm = Machine::new();
    xasm.push(Value::function(dict, &xasm));
    xasm.copy();
    xasm.push(Value::string("dict"));
    xasm.store();

    xasm.push(Value::function(list, &xasm));
    xasm.copy();
    xasm.push(Value::string("list"));
    xasm.store();
    xasm.push(Value::function(len, &xasm));
    xasm.copy();
    xasm.push(Value::string("len"));
    xasm.store();
    xasm.push(Value::function(push, &xasm));
    xasm.copy();
    xasm.push(Value::string("push"));
    xasm.store();
    xasm.push(Value::function(pop, &xasm));
    xasm.copy();
    xasm.push(Value::string("pop"));
    xasm.store();

    xasm.push(Value::function(print, &xasm));
    xasm.copy();
    xasm.push(Value::string("print"));
    xasm.store();
    xasm.push(Value::function(println, &xasm));
    xasm.copy();
    xasm.push(Value::string("println"));
    xasm.store();
    xasm.push(Value::function(new, &xasm));
    xasm.copy();
    xasm.push(Value::string("new"));
    xasm.store();

    xasm.push(Value::function(add, &xasm));
    xasm.copy();
    xasm.push(Value::string("add"));
    xasm.store();
    xasm.push(Value::function(sub, &xasm));
    xasm.copy();
    xasm.push(Value::string("sub"));
    xasm.store();
    xasm.push(Value::function(mul, &xasm));
    xasm.copy();
    xasm.push(Value::string("mul"));
    xasm.store();
    xasm.push(Value::function(div, &xasm));
    xasm.copy();
    xasm.push(Value::string("div"));
    xasm.store();
"#;

