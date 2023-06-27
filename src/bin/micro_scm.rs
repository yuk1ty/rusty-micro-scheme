use anyhow::Result;
use clap::Parser;
use rusty_micro_scheme::{
    command::{Mode, Opts},
    executor::{file_load::RunFileLoad, repl::Repl, Executor},
};

fn main() -> Result<()> {
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
