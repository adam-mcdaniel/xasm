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

        if let Err(_) = Command::new("go")
            .args(&["run", "main.go"])
            .current_dir(Self::build_dir()?)
            .stdout(Stdio::inherit())
            .output() {
			return Err(String::from("Could not run `go`, is golang properly installed?"))
		}

        Ok(())
    }

    fn build(compiled: &str) -> Result<(), String> {
        match create_dir_all(Self::build_dir()?) {
            _ => {}
        };

        write(Self::build_dir()? + "/main.go", compiled).expect("Could not write to file main.go");

		// Install xgopher
        if let Err(_) = Command::new("go")
            .args(&["get", "github.com/adam-mcdaniel/xgopher"])
            .current_dir(Self::build_dir()?)
            .stdout(Stdio::inherit())
            .output() {
			return Err(String::from("Could not run `go`, is golang properly installed?"))
		}

		// Build main.go
        if let Err(_) = Command::new("go")
            .args(&["build", "main.go"])
            .current_dir(Self::build_dir()?)
            .stdout(Stdio::inherit())
            .output() {
			return Err(String::from("Could not run `go`, is golang properly installed?"))
		}

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
	result := NewEmptyList().Slice()

	value := m.Pop()
	count := value.Number()

	for ; count > 0; count-- {
		result = append(result, m.Pop())
	}

	m.Push(NewList(result))
}

func rng(m *Machine) {
	result := NewEmptyList().Slice()

	lower := m.Pop().Number()
	upper := m.Pop().Number()

	for count := lower; count < upper; count++ {
		result = append(result, NewNumber(count))
	}

	m.Push(NewList(result))
}

func filter(m *Machine) {
	result := NewEmptyList().Slice()

	list := m.Pop().Slice()
	function := m.Pop()

	for _, item := range list {
		m.Push(item)
		m.Push(function)
		m.Call()
		if m.Pop().Bool() {
			result = append(result, item)
		}
	}

	m.Push(NewList(result))
}

func reduce(m *Machine) {
	list := m.Pop().Slice()
	function := m.Pop()
	accumulator := m.Pop()

	for _, item := range list {
		m.Push(accumulator)
		m.Push(item)
		m.Push(function)
		m.Call()
		accumulator = m.Pop()
	}

	m.Push(accumulator)
}

func map_fn(m *Machine) {
	result := NewEmptyList().Slice()

	list := m.Pop().Slice()
	function := m.Pop()

	for _, item := range list {
		m.Push(item)
		m.Push(function)
		m.Call()
		result = append(result, m.Pop())
	}

	m.Push(NewList(result))
}

func reverse(m *Machine) {
	a := m.Pop().Slice()

	for i := len(a)/2-1; i >= 0; i-- {
		opp := len(a)-1-i
		a[i], a[opp] = a[opp], a[i]
	}

	m.Push(NewList(a))
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
	l := value.Slice()
	s := value.Str()
	n := math.Max(float64(len(l)), float64(len(s)))
	m.Push(NewNumber(n))
}

func format(m *Machine) {
	m.Push(NewString(fmt.Sprintf("%v", m.Pop())))
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
	xasm.Push(NewFunction(rng, xasm.Duplicate()))
	xasm.Push(NewString("range"))
	xasm.Store()
	xasm.Push(NewFunction(filter, xasm.Duplicate()))
	xasm.Push(NewString("filter"))
	xasm.Store()
	xasm.Push(NewFunction(map_fn, xasm.Duplicate()))
	xasm.Push(NewString("map"))
	xasm.Store()
	xasm.Push(NewFunction(reduce, xasm.Duplicate()))
	xasm.Push(NewString("reduce"))
	xasm.Store()
	xasm.Push(NewFunction(reverse, xasm.Duplicate()))
	xasm.Push(NewString("reverse"))
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
	xasm.Push(NewFunction(format, xasm.Duplicate()))
	xasm.Push(NewString("format"))
	xasm.Store()
    
"#;
}
