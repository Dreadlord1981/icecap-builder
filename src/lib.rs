use cli::Arguments;

pub mod cli;
pub mod operation;

pub trait Instruction {
	fn run(&self, args: Arguments) -> Result<(), anyhow::Error>;
}
