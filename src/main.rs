use clap::Parser;
use icebuilder::{cli::Arguments, operation::{IcecapOperation, Operation, PortfolioOperation}, Instruction};

fn main() -> Result<(), anyhow::Error> {

	let args = Arguments::parse();

	if args.build_icecap {
		
		let o = IcecapOperation;

		o.run(args)?;
	}
	else if args.build_portfolio {
		
		let o = PortfolioOperation;

		o.run(args)?;
	}
	else {
		let o = Operation;

		o.run(args)?;
	}

	Ok(())
}
