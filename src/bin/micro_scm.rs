use anyhow::Result;
use chumsky::Parser as _;
use clap::Parser;
use ezio::prelude::*;
use rusty_micro_scheme::{
    command::{Mode, Opts},
    parser::parser,
};

fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.mode {
        Mode::Run(opt) => {
            let program = file::read(opt.file);
            println!("{:?}", parser().parse_recovery(program));
        }
        Mode::Repl => unimplemented!(),
    }
    Ok(())
}
