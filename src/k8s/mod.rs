use core::convert::TryFrom;

use anyhow::Result;
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
	let client_config = match cli_opts.context {
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

	info!(cluster_url = client_config.cluster_url.to_string().as_str());
	Client::try_from(client_config).map_err(|source| AppError::KubeError {
		context: "create the kube client".to_string(),
		source,
	})
}
