use k8s_openapi::api::core::v1::Service;
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

pub async fn get_svcs(client: Client, namespace: &Option<String>) -> ObjectList<Service> {
	let api_svc: Api<Service> = if let Some(ns) = namespace {
		info!("getting service from namespace: {}", ns);
		Api::namespaced(client, ns)
	} else {
		info!("getting service from all namespace");
		Api::all(client)
	};

	let svcs = api_svc
		.list(&ListParams::default())
		.await
		.map_err(|source| AppError::KubeError {
			context: "list service".to_string(),
			source,
		});

	svcs.unwrap()
}
