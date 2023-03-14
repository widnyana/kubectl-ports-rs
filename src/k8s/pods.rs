use k8s_openapi::api::core::v1::Pod;
use kube::{
	api::{
		ListParams,
		ObjectList,
	},
	Api,
	Client,
};
use tracing::info;

use crate::k8s::errors::AppError;

pub async fn get_pods(client: Client, namespace: &Option<String>) -> ObjectList<Pod> {
	let api_pods: Api<Pod> = if let Some(ns) = namespace {
		info!("getting pods from namespace: {}", ns);
		Api::namespaced(client, ns)
	} else {
		info!("getting pods from all namespace");
		Api::all(client)
	};

	let pods = api_pods
		.list(&ListParams::default())
		.await
		.map_err(|source| AppError::KubeError {
			context: "list pods".to_string(),
			source,
		});

	pods.unwrap()
}
