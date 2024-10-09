#![deny(rust_2018_idioms)]

use clap::Parser;
use tracing_bunyan_formatter::{
	BunyanFormattingLayer,
	JsonStorageLayer,
};
use tracing_subscriber::{
	filter::EnvFilter,
	layer::SubscriberExt,
	Registry,
};

use crate::{
	display::{
		render_pod_as_table,
		render_svc_as_table,
	},
	extractor::{
		get_ports_from_pods,
		get_ports_from_svc,
	},
	k8s::new_client,
	opts::{
		CliOpts,
		Kind,
	},
};

mod display;
mod extractor;
mod k8s;
mod opts;
mod schemas;
mod utils;

fn init_tracing() {
	std::env::set_var(
		"RUST_LOG",
		std::env::var("RUST_LOG").unwrap_or_else(|_| "none,kubectl_ports=error".to_string()),
	);

	let formatting_layer =
		BunyanFormattingLayer::new(env!("CARGO_CRATE_NAME").to_owned(), std::io::stderr);

	let subscriber = Registry::default()
		.with(EnvFilter::from_default_env())
		.with(JsonStorageLayer)
		.with(formatting_layer);
	tracing::subscriber::set_global_default(subscriber).unwrap();
}

#[tokio::main]
async fn main() {
	init_tracing();

	let mut cli_opts = CliOpts::parse();

	let client = match new_client(&cli_opts).await {
		Ok(kc) => kc,
		Err(err) => panic!("Problem connecting to cluster: {:?}", err),
	};

	// override current namespace
	if cli_opts.namespace.is_none() {
		cli_opts.namespace = Option::from(client.default_namespace().to_string());
	}

	if cli_opts.resource == Kind::Pod {
		let resources = get_ports_from_pods(client, &cli_opts.namespace).await;
		render_pod_as_table(&resources).await;
	} else if cli_opts.resource == Kind::Service || cli_opts.resource == Kind::Svc {
		let _resources = get_ports_from_svc(client, &cli_opts.namespace).await;
		render_svc_as_table(&_resources).await;
	}
}
