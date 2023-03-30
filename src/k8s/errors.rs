use thiserror::Error;

#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum AppError {
	#[error("Failed to run command '{cmd:?}': {output:?}")]
	CmdError {
		cmd: String,
		output: Option<std::process::Output>,
		source: Option<std::io::Error>,
	},

	#[error("Failed to perform action using {context}")]
	KubeError {
		context: String,
		source: kube::Error,
	},

	#[error("Failed load config for {context}")]
	KubeConfigError {
		context: String,
		source: kube::config::KubeconfigError,
	},

	#[error("Failed to infer config on {context}")]
	KubeInferConfigError {
		context: String,
		source: kube::config::InferConfigError,
	},
}
