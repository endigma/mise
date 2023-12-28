use crate::cli;
use eyre::Result;

/// Generate usage spec
#[derive(Debug, clap::Args)]
#[clap(verbatim_doc_comment)]
pub struct Usage {}

impl Usage {
    pub fn run(self) -> Result<()> {
        let spec: usage::Spec = cli::Cli::command().into();
        dbg!(&spec);
        println!("{}", spec);
        Ok(())
    }
}
