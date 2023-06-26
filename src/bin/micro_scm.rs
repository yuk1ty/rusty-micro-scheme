use anyhow::Result;
use ariadne::{Label, Report, ReportKind, Source};
use chumsky::{error::SimpleReason, Parser as _};
use clap::Parser;
use ezio::prelude::*;
use rusty_micro_scheme::{
    command::{Mode, Opts},
    parser::parser,
    repl::Repl,
};

fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.mode {
        Mode::Run(opt) => {
            let program = file::read(&opt.file);
            let (output, errors) = parser().parse_recovery(program.clone());
            if let Some(output) = output {
                println!("{:?}", output);
            }

            let filepath = opt.file.to_str().unwrap();
            for e in errors {
                let msg = match e.reason() {
                    SimpleReason::Custom(msg) => msg.clone(),
                    SimpleReason::Unexpected => format!(
                        "Unexpected {}",
                        e.found()
                            .map(|c| format!("token {}", c))
                            .unwrap_or_else(|| "EOF".to_string()),
                    ),
                    _ => unreachable!(),
                };
                Report::build(ReportKind::Error, filepath, e.span().start)
                    .with_label(Label::new((filepath, e.span())))
                    .with_message(msg)
                    .finish()
                    .print((filepath, Source::from(program.clone())))
                    .unwrap()
            }
        }
        Mode::Repl => {
            let mut repl = Repl::new("history.txt")?;
            repl.run()?;
        }
    }
    Ok(())
}
