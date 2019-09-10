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
	if list, err := m.Pop(); err == nil {
		if item, err := m.Pop(); err == nil {
			m.Push(NewList(append(list.Slice(), item)))
		}
	}
}

func pop(m *Machine) {
	if value, err := m.Pop(); err == nil {
		list := value.Slice()

		item, list := list[len(list)-1], list[:len(list)-1]
		m.Push(item)
		m.Push(NewList(list))
	}
}

func length(m *Machine) {
	if value, err := m.Pop(); err == nil {
		list := value.Slice()
		m.Push(NewNumber(float64(len(list))))
	}
}

func print(m *Machine) {
	if a, err := m.Pop(); err == nil {
		fmt.Print(*a)
	}
}

func println(m *Machine) {
	if a, err := m.Pop(); err == nil {
		fmt.Println(*a)
	}
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
	if a, err := m.Pop(); err == nil {
		if b, err := m.Pop(); err == nil {
			m.Push(NewNumber(a.Number() + b.Number()))
		}
	}
}

func sub(m *Machine) {
	if a, err := m.Pop(); err == nil {
		if b, err := m.Pop(); err == nil {
			m.Push(NewNumber(a.Number() - b.Number()))
		}
	}
}

func mul(m *Machine) {
	if a, err := m.Pop(); err == nil {
		if b, err := m.Pop(); err == nil {
			m.Push(NewNumber(a.Number() * b.Number()))
		}
	}
}

func div(m *Machine) {
	if a, err := m.Pop(); err == nil {
		if b, err := m.Pop(); err == nil {
			m.Push(NewNumber(a.Number() / b.Number()))
		}
	}
}

func rem(m *Machine) {
	if a, err := m.Pop(); err == nil {
		if b, err := m.Pop(); err == nil {
			m.Push(NewNumber(math.Mod(a.Number(), b.Number())))
		}
	}
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
	xasm.Push(NewFunction(debug, xasm.Duplicate()))
	xasm.Push(NewString("debug"))
	xasm.Store()
    
"#;
}
