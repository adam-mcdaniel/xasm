use std::fs::{create_dir_all, rename, write};
use std::path::Path;
use std::process::{Command, Stdio};
use fs_extra::dir::{copy, CopyOptions};
use xassembler::Golang;

use crate::compile::Compile;


fn gopath() -> Result<String, String> {
    if cfg!(unix) {
        Ok(format!("{}/go", Golang::home_dir()?))
    } else {
        Ok(String::from("C:\\Go"))
    }
}


impl Compile for Golang {
    fn compile_subcommand(compiled: &str, dependency_paths: Vec<&str>, output_path: &str) -> Result<(), String> {
        Self::build(compiled, dependency_paths)?;

        match (rename(Self::build_dir()? + "/main", output_path), 
				rename(Self::build_dir()? + "/main.exe", output_path),
				rename(Self::build_dir()? + "/main.bin", output_path)) {
			(Err(_), Err(_), Err(_)) => {
                return Err(format!("Could not move compiled executable to `{}`. Your program most likely did not compile successfully. Check your foreign functions' names", output_path))
			}
            _ => {}
        };

        Ok(())
    }

    fn run_subcommand(compiled: &str, dependency_paths: Vec<&str>) -> Result<(), String> {
        Self::build(compiled, dependency_paths)?;

        if let Err(_) = Command::new("go")
            .args(&["run", "main.go"])
            .current_dir(Self::build_dir()?)
            .stdout(Stdio::inherit())
            .output() {
			return Err(String::from("Could not run `go`, is golang properly installed?"))
		}

        Ok(())
    }

    fn build(compiled: &str, dependency_paths: Vec<&str>) -> Result<(), String> {
        match create_dir_all(Self::build_dir()?) {
            _ => {}
        };

		let mut copy_opts = CopyOptions::new();
		copy_opts.overwrite = true;

		for dep in &dependency_paths {
			// Get the path to the `src` folder in the GOPATH
			let import_path = format!("{}/src/", gopath()?);

			// Copy the dependency to the `src` folder in the GOPATH
			if let Err(_) = copy(dep, &import_path, &copy_opts) {
				return Err(format!("Could not copy package `{}` to `{}`", dep, import_path))
			};
		}

        let result = format!("package main\nimport (\n{}\n)\n{}", dependency_paths
                                        .iter()
                                        .map(|s|
                                            format!(". \"{}\"",
												Path::new(s).file_name().unwrap()
                                                            .to_str().unwrap()
											))
                                        .collect::<Vec<String>>()
                                        .join("\n"), compiled);

        write(Self::build_dir()? + "/main.go", result).expect("Could not write to file main.go");

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
    const PRELUDE: &'static str = r#"
import (
	. "github.com/adam-mcdaniel/xgopher"
)

func Xasm_dict(m *Machine) {
	m.Push(NewEmptyTree())
}

func main() {
	xasm := MakeMachine()
    xasm.Push(NewFunction(Xasm_dict, xasm))
    xasm.Push(NewString("dict"))
    xasm.Store()
	
"#;
}
