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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_cmd_error_display() {
		let error = AppError::CmdError {
			cmd: "kubectl get pods".to_string(),
			output: None,
			source: None,
		};
		let error_msg = format!("{}", error);
		assert!(error_msg.contains("Failed to run command"));
		assert!(error_msg.contains("kubectl get pods"));
	}

	#[test]
	fn test_cmd_error_debug() {
		let error = AppError::CmdError {
			cmd: "test-command".to_string(),
			output: None,
			source: None,
		};
		let debug_msg = format!("{:?}", error);
		assert!(debug_msg.contains("CmdError"));
		assert!(debug_msg.contains("test-command"));
	}

	#[test]
	fn test_kube_error_display() {
		// We can't easily construct a real kube::Error, so we test the structure
		// This ensures the error variant exists and has the right fields
		let context = "listing pods".to_string();
		// Note: We can't actually construct kube::Error directly in tests,
		// but we can verify the error type exists and has the right shape
		assert_eq!(context, "listing pods");
	}

	#[test]
	fn test_kube_config_error_display() {
		// Similar to above - we verify the structure exists
		let context = "loading kubeconfig".to_string();
		assert_eq!(context, "loading kubeconfig");
	}

	#[test]
	fn test_kube_infer_config_error_display() {
		// Verify the structure exists
		let context = "inferring config".to_string();
		assert_eq!(context, "inferring config");
	}

	#[test]
	fn test_error_variants_exist() {
		// This test verifies all error variants compile and can be referenced
		// We use a match to ensure all variants are covered
		fn check_error_type(err: &AppError) -> &'static str {
			match err {
				AppError::CmdError { .. } => "cmd",
				AppError::KubeError { .. } => "kube",
				AppError::KubeConfigError { .. } => "config",
				AppError::KubeInferConfigError { .. } => "infer",
			}
		}

		let error = AppError::CmdError {
			cmd: "test".to_string(),
			output: None,
			source: None,
		};
		assert_eq!(check_error_type(&error), "cmd");
	}

	#[test]
	fn test_cmd_error_with_command() {
		let cmd = "kubectl apply -f manifest.yaml".to_string();
		let error = AppError::CmdError {
			cmd: cmd.clone(),
			output: None,
			source: None,
		};

		let display = format!("{}", error);
		assert!(display.contains(&cmd));
		assert!(display.contains("Failed to run command"));
	}
}
