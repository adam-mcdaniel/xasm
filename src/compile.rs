use xassembler::{compile, Target};


pub trait Compile: Target {
    const BUILD_DIR_NAME: &'static str;
    const PRELUDE: &'static str;
    const TERMINATE: &'static str;
    fn compile_subcommand(compiled: &str, dependeny_paths: Vec<&str>, output_path: &str) -> Result<(), String>;
    fn run_subcommand(compiled: &str, dependeny_paths: Vec<&str>) -> Result<(), String>;
    fn build(compiled: &str, dependeny_paths: Vec<&str>) -> Result<(), String>;
    fn assemble(script: &str) -> Result<String, String>
    where
        Self: Sized,
    {
        Ok(format!(
            "{} {} {}",
            Self::PRELUDE,
            compile::<Self>(script)?,
            Self::TERMINATE
        ))
    }

    fn home_dir() -> Result<String, String> {
        let home = dirs::home_dir().ok_or_else(|| String::from("No home directory in this environment"))?;
        Ok(home
            .to_str()
            .ok_or_else(|| String::from("No home directory in this environment"))?
            .to_string())
    }

    fn build_dir() -> Result<String, String> {
        Ok(Self::home_dir()? + "/" + Self::BUILD_DIR_NAME)
    }
}
