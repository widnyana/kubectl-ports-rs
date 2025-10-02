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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_kind_default_is_pod() {
		// Test that the default Kind is Pod
		let default_kind = Kind::Pod;
		assert_eq!(default_kind, Kind::Pod);
	}

	#[test]
	fn test_kind_variants() {
		// Test all Kind variants exist and are distinct
		let pod = Kind::Pod;
		let service = Kind::Service;
		let svc = Kind::Svc;

		assert_eq!(pod, Kind::Pod);
		assert_eq!(service, Kind::Service);
		assert_eq!(svc, Kind::Svc);
		assert_ne!(pod, service);
		assert_ne!(pod, svc);
		assert_ne!(service, svc);
	}

	#[test]
	fn test_kind_clone() {
		let kind = Kind::Pod;
		let cloned = kind.clone();
		assert_eq!(kind, cloned);
	}

	#[test]
	fn test_kind_copy() {
		let kind = Kind::Service;
		let copied = kind;
		assert_eq!(kind, copied);
	}

	#[test]
	fn test_kind_debug() {
		// Test Debug trait implementation
		let kind = Kind::Pod;
		let debug_str = format!("{:?}", kind);
		assert!(debug_str.contains("Pod"));
	}

	#[test]
	fn test_cli_opts_structure() {
		// Test that CliOpts has the expected fields
		let opts = CliOpts {
			context: Some("test-context".to_string()),
			namespace: Some("default".to_string()),
			resource: Kind::Pod,
		};

		assert_eq!(opts.context, Some("test-context".to_string()));
		assert_eq!(opts.namespace, Some("default".to_string()));
		assert_eq!(opts.resource, Kind::Pod);
	}

	#[test]
	fn test_cli_opts_with_none_values() {
		let opts = CliOpts {
			context: None,
			namespace: None,
			resource: Kind::Service,
		};

		assert_eq!(opts.context, None);
		assert_eq!(opts.namespace, None);
		assert_eq!(opts.resource, Kind::Service);
	}

	#[test]
	fn test_cli_opts_with_svc_kind() {
		let opts = CliOpts {
			context: None,
			namespace: Some("kube-system".to_string()),
			resource: Kind::Svc,
		};

		assert_eq!(opts.namespace, Some("kube-system".to_string()));
		assert_eq!(opts.resource, Kind::Svc);
	}
}
