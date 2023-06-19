use anyhow::Result;
use clap::Parser;
use ezio::prelude::*;
use rusty_micro_scheme::{
    command::{Mode, Opts},
    lexer::tokenize,
};

fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.mode {
        Mode::Run(opt) => {
            let program = file::read(opt.file);
            println!("{:?}", tokenize(&mut program.chars().peekable()));
        }
        Mode::Repl => unimplemented!(),
    }
    Ok(())
}
