use std::fs::{rename, write};
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

impl Compile for Rust {
    fn compile_subcommand(compiled: &str, output_path: &str) -> Result<(), String> {
        Self::build(compiled)?;

        match rename(Self::build_dir()? + "/target/release/target", output_path) {
            _ => {}
        };
        match rename(
            Self::build_dir()? + "/target/release/target.exe",
            output_path,
        ) {
            _ => {}
        };
        match rename(
            Self::build_dir()? + "/target/release/target.bin",
            output_path,
        ) {
            _ => {}
        };

        Ok(())
    }

    fn run_subcommand(compiled: &str) -> Result<(), String> {
        Self::build(compiled)?;

        Command::new("cargo")
            .args(&["run", "--release"])
            .current_dir(Self::build_dir()?)
            .stdout(Stdio::inherit())
            .output()
            .unwrap();

        Ok(())
    }

    fn build(compiled: &str) -> Result<(), String> {
        match Command::new("cargo")
            .current_dir(Self::home_dir()?)
            .args(&["new", Self::BUILD_DIR_NAME])
            .output()
        {
            _ => (),
        };

        write(Self::build_dir()? + "/src/main.rs", compiled)
            .expect("Could not write to file target/src/main.rs");

        write(Self::build_dir()? + "/Cargo.toml", MANIFEST)
            .expect("Could not write to target/Cargo.toml");

        Command::new("cargo")
            .args(&["build", "--release"])
            .current_dir(Self::build_dir()?)
            .output()
            .unwrap();

        Ok(())
    }

    const TERMINATE: &'static str = "\n}";
    const BUILD_DIR_NAME: &'static str = "xasm_build";
    const PRELUDE: &'static str = r#"
extern crate xmachine;
use xmachine::{Machine, Value};


fn dict(xasm: &mut Machine) {
    xasm.push(Value::tree());
}

fn list(xasm: &mut Machine) {
    let mut result = vec![];
    let val = xasm.get_arg();
    let count = i32::from(val.clone());
    
    for _ in 0..count {
        result.push(xasm.pop());
    }

    xasm.return_value(Value::List(result));
}

fn range(xasm: &mut Machine) {
    let mut result = vec![];
    let lower = i32::from(xasm.get_arg());
    let upper = i32::from(xasm.get_arg());
    
    for i in lower..upper {
        result.push(Value::number(i));
    }
    xasm.return_value(Value::List(result));
}

fn filter(xasm: &mut Machine) {
    let mut result = vec![];
    let list = xasm.get_arg();
    let function = xasm.pop();

    if let Value::List(mut l) = list {
        for item in l {
            xasm.push(item.clone());
            xasm.push(function.clone());
            xasm.call();
            if bool::from(xasm.get_arg()) {
                result.push(item);
            }
        }
    }

    xasm.return_value(Value::List(result));
}

fn reverse(xasm: &mut Machine) {
    let mut list = xasm.get_arg();

    if let Value::List(mut l) = list {
        l.reverse();
        list = Value::List(l);
    }

    xasm.return_value(list);
}

fn reduce(xasm: &mut Machine) {
    let list = xasm.get_arg();
    let function = xasm.pop();
    let mut accumulator = xasm.pop();

    if let Value::List(mut l) = list {
        for item in l {
            xasm.push(accumulator.clone());
            xasm.push(item.clone());
            xasm.push(function.clone());
            xasm.call();
            accumulator = xasm.pop();
        }
    }

    xasm.push(accumulator);
}

fn map_fn(xasm: &mut Machine) {
    let mut result = vec![];

    let list = xasm.get_arg();
    let function = xasm.pop();

    if let Value::List(mut l) = list {
        for item in l {
            xasm.push(item.clone());
            xasm.push(function.clone());
            xasm.call();
            result.push(xasm.pop());
        }
    }

    xasm.return_value(Value::List(result));
}

fn push(xasm: &mut Machine) {
    let list_value = xasm.pop();
    if let Value::List(mut l) = (*list_value).clone() {
        l.push(xasm.pop());
        xasm.return_value(Value::List(l));
    }
}

fn pop(xasm: &mut Machine) {
    let value = xasm.pop();
    if let Value::List(mut l) = (*value).clone() {
        let last_value = l[l.len() - 1].clone();
        l.pop();
        xasm.push(last_value.copy());
        xasm.return_value(Value::List(l));
    }
}

fn len(xasm: &mut Machine) {
    let value = xasm.pop();
    xasm.push(
        Value::number(
            if let Value::List(l) = (*value).clone() {
                l.len() as f64
            } else if let Value::String(s) = (*value).clone() {
                s.len() as f64
            } else {
                0.0
            }
        )
    );
}

fn format(xasm: &mut Machine) {
    let s = Value::string(format!("{}", xasm.pop()));
    xasm.push(s);
}

fn print(xasm: &mut Machine) {
    print!("{}", xasm.pop());
}

fn println(xasm: &mut Machine) {
    println!("{}", xasm.pop());
}

fn debug(xasm: &mut Machine) {
    println!("{}", xasm);
}

fn new(xasm: &mut Machine) {
    let class = xasm.pop();
    class.call(xasm);
    xasm.push(Value::string("new"));
    xasm.method_call();
}

fn add(xasm: &mut Machine) {
    let first = xasm.get_arg();
    let second = xasm.get_arg();
    xasm.return_value(first + second);
}

fn sub(xasm: &mut Machine) {
    let first = xasm.get_arg();
    let second = xasm.get_arg();
    xasm.return_value(first - second);
}

fn mul(xasm: &mut Machine) {
    let first = xasm.get_arg();
    let second = xasm.get_arg();
    xasm.return_value(first * second);
}

fn div(xasm: &mut Machine) {
    let first = xasm.get_arg();
    let second = xasm.get_arg();
    xasm.return_value(first / second);
}

fn rem(xasm: &mut Machine) {
    let first = xasm.get_arg();
    let second = xasm.get_arg();
    xasm.return_value(first % second);
}

fn not(xasm: &mut Machine) {
    let value = xasm.get_arg();
    xasm.return_value(!value);
}

fn eq(xasm: &mut Machine) {
    let first = xasm.get_arg();
    let second = xasm.get_arg();
    xasm.return_value(Value::from(first == second));
}

fn is_error(xasm: &mut Machine) {
    match xasm.get_arg() {
        Value::Error(_) => xasm.return_value(Value::from(true)),
        _ => xasm.return_value(Value::from(false))
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
    xasm.push(Value::function(range, &xasm));
    xasm.copy();
    xasm.push(Value::string("range"));
    xasm.store();
    xasm.push(Value::function(filter, &xasm));
    xasm.copy();
    xasm.push(Value::string("filter"));
    xasm.store();
    xasm.push(Value::function(reduce, &xasm));
    xasm.copy();
    xasm.push(Value::string("reduce"));
    xasm.store();
    xasm.push(Value::function(map_fn, &xasm));
    xasm.copy();
    xasm.push(Value::string("map"));
    xasm.store();
    xasm.push(Value::function(reverse, &xasm));
    xasm.copy();
    xasm.push(Value::string("reverse"));
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
    xasm.push(Value::function(rem, &xasm));
    xasm.copy();
    xasm.push(Value::string("rem"));
    xasm.store();
    xasm.push(Value::function(not, &xasm));
    xasm.copy();
    xasm.push(Value::string("not"));
    xasm.store();
    xasm.push(Value::function(debug, &xasm));
    xasm.copy();
    xasm.push(Value::string("debug"));
    xasm.store();
    xasm.push(Value::function(eq, &xasm));
    xasm.copy();
    xasm.push(Value::string("eq"));
    xasm.store();
    xasm.push(Value::function(is_error, &xasm));
    xasm.copy();
    xasm.push(Value::string("is_error"));
    xasm.store();
    xasm.push(Value::function(format, &xasm));
    xasm.copy();
    xasm.push(Value::string("format"));
    xasm.store();
"#;
}
