use std::fs::{create_dir_all, rename, write};
use std::process::{Command, Stdio};
use xassembler::Golang;

use crate::compile::Compile;

impl Compile for Golang {
    fn compile_subcommand(compiled: &str, output_path: &str) -> Result<(), String> {
        Self::build(compiled)?;

        rename(Self::build_dir()? + "/main", output_path).unwrap();

        Ok(())
    }

    fn run_subcommand(compiled: &str) -> Result<(), String> {
        Self::build(compiled)?;

        Command::new("go")
            .args(&["run", "main.go"])
            .current_dir(Self::build_dir()?)
            .stdout(Stdio::inherit())
            .output()
            .unwrap();

        Ok(())
    }

    fn build(compiled: &str) -> Result<(), String> {
        match create_dir_all(Self::build_dir()?) {
            _ => {}
        };

        write(Self::build_dir()? + "/main.go", compiled).expect("Could not write to file main.go");

        Command::new("go")
            .args(&["build", "main.go"])
            .current_dir(Self::build_dir()?)
            .stdout(Stdio::inherit())
            .output()
            .unwrap();

        Ok(())
    }

    const TERMINATE: &'static str = "\n}";
    const BUILD_DIR_NAME: &'static str = "xasm_go_build";
    const PRELUDE: &'static str = r#"package main

import (
	"fmt"
	"math"
	. "github.com/adam-mcdaniel/xgopher"
)

func dict(m *Machine) {
	m.Push(NewEmptyTree())
}

func list(m *Machine) {
	m.Push(NewEmptyList())
}

func push(m *Machine) {
	list := m.Pop().Slice()
	item := m.Pop()

	m.Push(NewList(append(list, item)))
}

func pop(m *Machine) {
	list := m.Pop().Slice()

	item, list := list[len(list)-1], list[:len(list)-1]
	m.Push(item)
	m.Push(NewList(list))
}

func length(m *Machine) {
	m.Push(NewNumber(float64(len(m.Pop().Slice()))))
}

func print(m *Machine) {
	fmt.Print(*m.Pop())
}

func println(m *Machine) {
	fmt.Println(*m.Pop())
}

func new(m *Machine) {
	m.Call()
	m.Push(NewString("new"))
	m.MethodCall()
}

func add(m *Machine) {
	a := m.Pop()
	b := m.Pop()

	m.Push(NewNumber(a.Number() + b.Number()))
}

func sub(m *Machine) {
	a := m.Pop()
	b := m.Pop()

	m.Push(NewNumber(a.Number() - b.Number()))
}

func mul(m *Machine) {
	a := m.Pop()
	b := m.Pop()

	m.Push(NewNumber(a.Number() * b.Number()))
}

func div(m *Machine) {
	a := m.Pop()
	b := m.Pop()

	m.Push(NewNumber(a.Number() / b.Number()))
}

func rem(m *Machine) {
	a := m.Pop()
	b := m.Pop()

	m.Push(NewNumber(math.Mod(a.Number(), b.Number())))
}

func main() {
	xasm := MakeMachine()
	xasm.Push(NewFunction(dict, xasm))
	xasm.Push(NewString("dict"))
	xasm.Store()
	xasm.Push(NewFunction(list, xasm))
	xasm.Push(NewString("list"))
	xasm.Store()
	xasm.Push(NewFunction(length, xasm))
	xasm.Push(NewString("len"))
	xasm.Store()
	xasm.Push(NewFunction(push, xasm))
	xasm.Push(NewString("push"))
	xasm.Store()
	xasm.Push(NewFunction(pop, xasm))
	xasm.Push(NewString("pop"))
	xasm.Store()
	xasm.Push(NewFunction(print, xasm))
	xasm.Push(NewString("print"))
	xasm.Store()
	xasm.Push(NewFunction(println, xasm))
	xasm.Push(NewString("println"))
	xasm.Store()
	xasm.Push(NewFunction(new, xasm))
	xasm.Push(NewString("new"))
	xasm.Store()
	xasm.Push(NewFunction(add, xasm))
	xasm.Push(NewString("add"))
	xasm.Store()
	xasm.Push(NewFunction(sub, xasm))
	xasm.Push(NewString("sub"))
	xasm.Store()
	xasm.Push(NewFunction(mul, xasm))
	xasm.Push(NewString("mul"))
	xasm.Store()
	xasm.Push(NewFunction(div, xasm))
	xasm.Push(NewString("div"))
	xasm.Store()
	xasm.Push(NewFunction(rem, xasm))
	xasm.Push(NewString("rem"))
	xasm.Store()
    
"#;
}
