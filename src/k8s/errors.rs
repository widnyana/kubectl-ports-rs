#[derive(thiserror::Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum AppError {
	#[error("Failed to run '{cmd}'")]
	CmdError {
		cmd: String,
		output: Option<std::process::Output>,
		source: Option<std::io::Error>,
	},

	#[error("Failed to {context}")]
	KubeError {
		context: String,
		source: kube::Error,
	},

	#[error("Failed to {context}")]
	KubeConfigError {
		context: String,
		source: kube::config::KubeconfigError,
	},

	#[error("Failed to {context}")]
	KubeInferConfigError {
		context: String,
		source: kube::config::InferConfigError,
	},
}
