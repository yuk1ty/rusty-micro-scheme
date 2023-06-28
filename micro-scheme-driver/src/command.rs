use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "micro_sm", 
          about = "A tiny scheme compiler.", 
          version = "0.1.0", 
          long_about = None
)]
pub struct Opts {
    #[command(subcommand)]
    pub mode: Mode,
}

#[derive(Subcommand, Debug)]
pub enum Mode {
    /// Launch REPL mode
    Repl,
    /// Run the compiler by reading files
    Run(BuildOpt),
}

#[derive(Args, Debug)]
pub struct BuildOpt {
    /// Compile target file path
    #[arg(short, long)]
    pub file: PathBuf,
}

#[cfg(test)]
mod tests {
    use crate::command::Opts;

    #[test]
    fn verify() {
        use clap::CommandFactory;
        Opts::command().debug_assert();
    }
}
