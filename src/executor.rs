use anyhow::Result;

pub mod file_load;
pub mod repl;

pub trait Executor {
    fn run(self) -> Result<()>;
}
