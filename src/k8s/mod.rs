use core::convert::TryFrom;
use std::{
	env,
	time::Duration,
};

use anyhow::Result;
use http::Uri;
use kube::Client;
use tracing::{
	info,
	instrument,
};

use crate::{
	k8s::errors::AppError,
	opts::CliOpts,
};

pub mod errors;
pub mod pods;
pub mod service;

#[instrument]
async fn refresh_kube_config(context: &Option<String>) -> Result<(), AppError> {
	// HACK force refresh token by calling "kubectl cluster-info before loading
	// configuration"
	info!("refreshing kube config");
	use std::process::Command;
	let mut cmd = Command::new("kubectl");
	cmd.arg("cluster-info");
	if let Some(ref context) = context {
		cmd.arg("--context").arg(context);
	}

	let output = cmd.output().map_err(|source| AppError::CmdError {
		cmd: "kubectl cluster-info".to_owned(),
		output: None,
		source: Some(source),
	})?;
	if !output.status.success() {
		return Err(AppError::CmdError {
			cmd: "kubectl cluster-info".to_owned(),
			output: Some(output),
			source: None,
		});
	}
	Ok(())
}

#[instrument]
pub async fn new_client(cli_opts: &CliOpts) -> Result<Client, AppError> {
	refresh_kube_config(&cli_opts.context).await?;

	let mut client_config = match cli_opts.context {
		Some(ref context) => kube::Config::from_kubeconfig(&kube::config::KubeConfigOptions {
			context: Some(context.clone()),
			..Default::default()
		})
		.await
		.map_err(|source| AppError::KubeConfigError {
			context: "create the kube client config".to_string(),
			source,
		})?,
		None => kube::Config::infer().await.map_err(|source| AppError::KubeInferConfigError {
			context: "create the kube client config".to_string(),
			source,
		})?,
	};

	if let Ok(https_proxy) = env::var("HTTPS_PROXY") {
		info!("Using HTTPS_PROXY: {}", https_proxy);
		client_config.proxy_url = https_proxy.parse::<Uri>().ok();
	}

	// lower down the timeout
	client_config.connect_timeout = Some(Duration::from_secs(30));
	client_config.read_timeout = Some(Duration::from_secs(15));

	Client::try_from(client_config).map_err(|source| AppError::KubeError {
		context: "create the kube client".to_string(),
		source,
	})
}
