use super::*;

#[derive(Subcommand)]
pub enum DevSubcommand {
}

impl DevSubcommand {
    pub fn process(self) -> Result<(), Error> {
        Ok(())
    }
}