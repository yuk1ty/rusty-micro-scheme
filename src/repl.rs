use std::path::Path;

use anyhow::Result;
use ariadne::{Label, Report, ReportKind, Source};
use chumsky::{error::SimpleReason, Parser};
use rustyline::{error::ReadlineError, DefaultEditor};

use crate::parser::parser;

static EXIT: &str = "exit";

pub struct Repl<P: AsRef<Path>> {
    history_file_path: P,
    editor: rustyline::DefaultEditor,
}

impl<P: AsRef<Path>> Repl<P> {
    pub fn new(history_file_path: P) -> Result<Self> {
        let mut editor = DefaultEditor::new()?;
        if editor.load_history(&history_file_path).is_err() {
            println!("No previous history.");
        }

        Ok(Self {
            history_file_path: history_file_path,
            editor,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            let readline = self.editor.readline(">> ");
            match readline {
                Ok(line) if line == EXIT => {
                    println!("Bye!");
                    break;
                }
                Ok(line) => {
                    self.editor.add_history_entry(line.as_str())?;
                    let (output, errors) = parser().parse_recovery(line.clone());
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
                        let src_id = "repl";
                        Report::build(ReportKind::Error, src_id, e.span().start)
                            .with_label(Label::new((src_id, e.span())))
                            .with_message(msg)
                            .finish()
                            .print((src_id, Source::from(line.clone())))
                            .unwrap()
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
        self.editor.save_history(&self.history_file_path)?;
        Ok(())
    }
}
