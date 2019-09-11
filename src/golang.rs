use std::fs::{create_dir_all, rename, write};
use std::process::{Command, Stdio};
use xassembler::Golang;

use crate::compile::Compile;

impl Compile for Golang {
    fn compile_subcommand(compiled: &str, output_path: &str) -> Result<(), String> {
        Self::build(compiled)?;

        match rename(Self::build_dir()? + "/main", output_path) {
            _ => {}
        };
        match rename(Self::build_dir()? + "/main.exe", output_path) {
            _ => {}
        };
        match rename(Self::build_dir()? + "/main.bin", output_path) {
            _ => {}
        };

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
	. "github.com/adam-mcdaniel/xgopher"
)

func dict(m *Machine) {
	m.Push(NewEmptyTree())
}

func list(m *Machine) {
	m.Push(NewEmptyList())
}

func push(m *Machine) {
	list := m.Pop()
	item := m.Pop()
	m.Push(NewList(append(list.Slice(), item)))
}

func pop(m *Machine) {
	value := m.Pop()
	list := value.Slice()
	if len(list) > 0 {
		item := list[len(list)-1]
		list = list[:len(list)-1]
		m.Push(item)
	} else {
		m.Push(NewNone())
	}
	m.Push(NewList(list))
}

func length(m *Machine) {
	value := m.Pop()
	list := value.Slice()
	m.Push(NewNumber(float64(len(list))))
}

func print(m *Machine) {
	fmt.Print(*m.Pop())
}

func println(m *Machine) {
	fmt.Println(*m.Pop())
}

func debug(m *Machine) {
	fmt.Println(*m)
}

func new(m *Machine) {
	m.Call()
	m.Push(NewString("new"))
	m.MethodCall()
}

func add(m *Machine) {
	a := m.Pop()
	b := m.Pop()
	m.Push(a.Add(b))
}

func sub(m *Machine) {
	a := m.Pop()
	b := m.Pop()
	m.Push(a.Sub(b))
}

func mul(m *Machine) {
	a := m.Pop()
	b := m.Pop()
	m.Push(a.Mul(b))
}

func div(m *Machine) {
	a := m.Pop()
	b := m.Pop()
	m.Push(a.Div(b))
}

func rem(m *Machine) {
	a := m.Pop()
	b := m.Pop()
	m.Push(a.Rem(b))
}

func not(m *Machine) {
	m.Push(m.Pop().Not())
}

func eq(m *Machine) {
	m.Push(m.Pop().Eq(m.Pop()))
}

func is_error(m *Machine) {
	m.Push(NewBool(m.Pop().Type() == ErrorType))
}

func main() {
	xasm := MakeMachine()
	xasm.Push(NewFunction(dict, xasm.Duplicate()))
	xasm.Push(NewString("dict"))
	xasm.Store()
	xasm.Push(NewFunction(list, xasm.Duplicate()))
	xasm.Push(NewString("list"))
	xasm.Store()
	xasm.Push(NewFunction(length, xasm.Duplicate()))
	xasm.Push(NewString("len"))
	xasm.Store()
	xasm.Push(NewFunction(push, xasm.Duplicate()))
	xasm.Push(NewString("push"))
	xasm.Store()
	xasm.Push(NewFunction(pop, xasm.Duplicate()))
	xasm.Push(NewString("pop"))
	xasm.Store()
	xasm.Push(NewFunction(print, xasm.Duplicate()))
	xasm.Push(NewString("print"))
	xasm.Store()
	xasm.Push(NewFunction(println, xasm.Duplicate()))
	xasm.Push(NewString("println"))
	xasm.Store()
	xasm.Push(NewFunction(new, xasm.Duplicate()))
	xasm.Push(NewString("new"))
	xasm.Store()
	xasm.Push(NewFunction(add, xasm.Duplicate()))
	xasm.Push(NewString("add"))
	xasm.Store()
	xasm.Push(NewFunction(sub, xasm.Duplicate()))
	xasm.Push(NewString("sub"))
	xasm.Store()
	xasm.Push(NewFunction(mul, xasm.Duplicate()))
	xasm.Push(NewString("mul"))
	xasm.Store()
	xasm.Push(NewFunction(div, xasm.Duplicate()))
	xasm.Push(NewString("div"))
	xasm.Store()
	xasm.Push(NewFunction(rem, xasm.Duplicate()))
	xasm.Push(NewString("rem"))
	xasm.Store()
	xasm.Push(NewFunction(not, xasm.Duplicate()))
	xasm.Push(NewString("not"))
	xasm.Store()
	xasm.Push(NewFunction(debug, xasm.Duplicate()))
	xasm.Push(NewString("debug"))
	xasm.Store()
	xasm.Push(NewFunction(eq, xasm.Duplicate()))
	xasm.Push(NewString("eq"))
	xasm.Store()
    
"#;
}
