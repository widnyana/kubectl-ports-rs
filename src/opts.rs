use clap::Parser;

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
}
