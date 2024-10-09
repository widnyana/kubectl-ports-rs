use clap::{
	crate_authors,
	Parser,
	ValueEnum,
};

#[derive(Parser, Debug)]
#[clap(
    // global_settings(&[AppSettings::ColoredHelp, AppSettings::VersionlessSubcommands]),
    author = crate_authors!(),
	name = env!("CARGO_PKG_NAME"),
	version = env!("CARGO_PKG_VERSION"),
	about = "Show exposed container ports in the cluster.",
	long_about = "This tool allows you to easily view exposed container ports in your Kubernetes cluster. It provides a quick overview of which ports are open on your pods and services, helping you to monitor and troubleshoot your cluster's network configuration.

Features:
- List exposed ports for pods and services
- Filter results by namespace
- Support for custom kubeconfig contexts
- Detailed output including container names, port numbers, and protocols

Use this tool to quickly identify open ports, verify your service configurations, and ensure your cluster's network setup meets your expectations and security requirements."
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
