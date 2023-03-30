use clap::{
	Parser,
	ValueEnum,
};

#[derive(Parser, Debug)]
#[clap(
    // global_settings(&[AppSettings::ColoredHelp, AppSettings::VersionlessSubcommands]),
    author = env!("CARGO_PKG_HOMEPAGE"), about, version, long_about = None
)]
pub struct CliOpts {
	/// The name of the kubeconfig context to use
	#[clap(long, value_parser)]
	pub context: Option<String>,

	/// Show only pods from this namespace
	#[clap(short, long, value_parser)]
	pub namespace: Option<String>,

	#[clap(
		value_enum,
		default_value_t = Kind::Pod,
		help = String::from("select Kubernetes resource you want to list the port(s) ")
	)]
	pub resource: Kind,
}

#[derive(ValueEnum, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Kind {
	/// (default) Enable both coloring and formatting
	Pod,
	/// Apply syntax highlighting to output
	Service,
	Svc,
}
// impl Kind {
// 	pub fn as_str(&self) -> &'static str {
// 		match self {
// 			Kind::Pod => "pod",
// 			Kind::Service => "svc",
// 		}
// 	}
// }
