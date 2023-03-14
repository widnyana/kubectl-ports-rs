use k8s_openapi::api::core::v1::Pod;
use kube::{
	api::ObjectList,
	Client,
	ResourceExt,
};
use tracing::instrument;

use crate::{
	k8s::{
		errors::AppError,
		pods::get_pods,
	},
	schemas::{
		ContainerPort,
		Resource,
	},
	utils::is_scheduled,
};

#[instrument(skip(c))]
pub async fn get_ports_from_pods(c: Client, namespace: &Option<String>) -> Vec<Resource> {
	let mut resources: Vec<Resource> = vec![];

	process_ports(get_pods(c, namespace).await, &mut resources)
		.await
		.expect("could not process ports");

	resources
}

#[instrument(skip(out, pods))]
async fn process_ports(pods: ObjectList<Pod>, out: &mut Vec<Resource>) -> Result<(), AppError> {
	for pod in pods.into_iter().filter(is_scheduled) {
		let mut tmp = Resource::new(
			"pod".to_string(),
			pod.name_any(),
			pod.namespace().clone().unwrap_or_default(),
		);

		let spec = pod.spec.as_ref();
		let containers = spec.map(|s| s.containers.clone()).unwrap_or_default();
		for container in containers {
			if let Some(ports) = container.ports {
				for port in ports {
					tmp.ports.push(ContainerPort {
						container_name: container.name.clone(),
						container_port: port.container_port,
						host_ip: port.host_ip.unwrap_or_default(),
						host_port: port.host_port.unwrap_or_default(),
						name: port.name.unwrap_or_default(),
						protocol: port.protocol.unwrap_or("TCP".to_string()),
					});
				}
			}
		}

		out.push(tmp);
	}

	Ok(())
}
