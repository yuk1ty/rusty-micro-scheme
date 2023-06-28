mod command;
mod file_load;
mod repl;

use anyhow::Result;
use clap::Parser;
use command::{Mode, Opts};
use file_load::RunFileLoad;
use repl::Repl;

pub trait Executor {
    fn run(self) -> Result<()>;
}

pub fn boot() -> Result<()> {
    let opts = Opts::parse();
    match opts.mode {
        Mode::Run(opt) => {
            let run_file_load = RunFileLoad::new(opt.file);
            run_file_load.run()
        }
        Mode::Repl => {
            let repl = Repl::new("history.txt")?;
            repl.run()
        }
    }
}
