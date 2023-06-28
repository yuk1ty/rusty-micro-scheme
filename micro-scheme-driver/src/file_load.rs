use anyhow::Result;
use ezio::prelude::*;
use micro_scheme_compiler::run;
use std::path::PathBuf;

use super::Executor;

pub struct RunFileLoad {
    filepath: PathBuf,
}

impl RunFileLoad {
    pub fn new(filepath: PathBuf) -> Self {
        Self { filepath }
    }
}

impl Executor for RunFileLoad {
    fn run(self) -> Result<()> {
        let source = file::read(&self.filepath);
        run(self.filepath.to_str().unwrap(), source)
    }
}
