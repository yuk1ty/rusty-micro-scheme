pub mod ast;
pub mod ir;
mod parser;

use ansi_term::Colour;
use anyhow::Result;
use ariadne::{Label, Report, ReportKind, Source};
use chumsky::{error::SimpleReason, Parser};

use crate::{ir::ir_codegen, parser::parser};

pub fn run(path: &str, source: String) -> Result<()> {
    let (output, errors) = parser().parse_recovery(source.clone());
    if let Some(output) = output {
        println!("{} 🔥 -> {:?}", Colour::Green.paint("Compiled"), output);

        let ir = ir_codegen(output);
        println!("{} 🔥 -> {:?}", Colour::Yellow.paint("IR      "), ir);
    }

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
        Report::build(ReportKind::Error, path, e.span().start)
            .with_label(Label::new((path, e.span())))
            .with_message(msg)
            .finish()
            .print((path, Source::from(source.clone())))?;
    }

    Ok(())
}
