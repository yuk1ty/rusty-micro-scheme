use anyhow::Result;
use ariadne::{Label, Report, ReportKind, Source};
use chumsky::{error::SimpleReason, Parser};

use crate::parser::parser;

pub fn run(path: &str, source: String) -> Result<()> {
    let (output, errors) = parser().parse_recovery(source.clone());
    if let Some(output) = output {
        println!("{:?}", output);
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
