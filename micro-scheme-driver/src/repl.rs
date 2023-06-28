use std::path::Path;

use anyhow::Result;
use micro_scheme_compiler::run;
use rustyline::{error::ReadlineError, DefaultEditor};

use super::Executor;

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
            history_file_path,
            editor,
        })
    }
}

impl<P: AsRef<Path>> Executor for Repl<P> {
    fn run(mut self) -> Result<()> {
        loop {
            let readline = self.editor.readline(">> ");
            match readline {
                Ok(line) if line == EXIT => {
                    println!("Bye!");
                    break;
                }
                Ok(line) => {
                    self.editor.add_history_entry(line.as_str())?;
                    run("repl", line)?;
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
