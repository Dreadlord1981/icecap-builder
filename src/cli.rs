use clap::Parser;
use serde::Serialize;

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all="kebab-case")]
pub enum BuildType {
	#[default]
	All
}

#[derive(Debug, Parser)]
#[command(author, version, about = "Utility to build lastest version of icecap services")]
#[clap(version, disable_help_flag=true)]
pub struct Arguments {

	#[arg(short('h'), long("host"), help="Remote host address", default_value="dksrv206")]
	pub host: String,

	#[arg(short('u'), long("user"), help="Remote host user")]
	pub user: String,

	#[arg(short('w'), long("password"), help="Remote host password")]
	pub password: String,

	#[arg(short('i'), long("ifs"), help="IFS on remote host to icecap project", default_value="/prj/icecap")]
	pub ifs: String,

	#[arg(short('t'), long("type"), help="Type to build", default_value_t, value_enum)]
	pub build: BuildType,

	#[arg(short('d'), long("deploy"), help="Deploy libl", default_value="CAPQA")]
	pub deploy: String,

	#[arg(short('b'), long("bin"), help="Bin libl", default_value="CAPDEV")]
	pub bin: String,

	#[arg(short('r'), long("release"), help="Release", default_value="false")]
	pub release: bool,

	#[arg(long("build-icecap"), help="Build icecap", default_value="false")]
	pub build_icecap: bool,

	#[arg(long("build-portfolio"), help="Build portfolio", default_value="false")]
	pub build_portfolio: bool,

	#[arg(short('H'), long, action = clap::ArgAction::Help)]
	pub help: Option<bool>
}