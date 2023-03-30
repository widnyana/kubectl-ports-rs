use k8s_openapi::api::core::v1::{
	Pod,
	Service,
};
use kube::{
	api::ObjectList,
	Client,
	ResourceExt,
};
use tracing::{
	error,
	instrument,
};

use crate::{
	k8s::{
		errors::AppError,
		pods::get_pods,
		service::get_svcs,
	},
	schemas::{
		ContainerPort,
		Resource,
		ServicePort,
		ServiceResource,
	},
	utils::{
		from_int_or_string_tostring,
		is_scheduled,
	},
};

#[instrument(skip(c))]
pub async fn get_ports_from_pods(c: Client, namespace: &Option<String>) -> Vec<Resource> {
	let mut resources: Vec<Resource> = vec![];

	process_ports_from_pods(get_pods(c, namespace).await, &mut resources)
		.await
		.expect("could not process ports");

	resources
}

#[instrument(skip(out, pods))]
async fn process_ports_from_pods(
	pods: ObjectList<Pod>,
	out: &mut Vec<Resource>,
) -> Result<(), AppError> {
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

//

#[instrument(skip(c))]
pub async fn get_ports_from_svc(c: Client, namespace: &Option<String>) -> Vec<ServiceResource> {
	let mut resources: Vec<ServiceResource> = vec![];

	process_ports_from_svcs(get_svcs(c, namespace).await, &mut resources)
		.await
		.expect("could not process ports");

	resources
}

#[instrument(skip(out, svcs))]
async fn process_ports_from_svcs(
	svcs: ObjectList<Service>,
	out: &mut Vec<ServiceResource>,
) -> Result<(), AppError> {
	for svc in svcs {
		let name = svc.name_any();
		let namespace = svc.namespace().unwrap_or_default();

		let svc_spec = if let Some(ss) = svc.spec.as_ref() {
			ss.clone()
		} else {
			error!(
				"got empty service spec on service with name: {:?} on namespace: {:?}. skipping",
				name, namespace
			);
			continue;
		};

		let mut tmp = ServiceResource::new(
			"service".to_string(),
			name.clone(),
			namespace.clone(),
			svc_spec.type_.unwrap_or_default(),
			svc_spec.cluster_ip.unwrap_or_default(),
			svc_spec.external_traffic_policy.unwrap_or_default(),
		);
		let ports = match svc_spec.ports.as_ref() {
			None => {
				error!(
					"got empty service ports on service with name: {:?} on namespace: {:?}. \
					skipping",
					name, namespace,
				);
				continue;
			}
			Some(p) => p.clone(),
		};

		for port in ports {
			tmp.ports.push(ServicePort {
				port_name: port.name.unwrap_or("-".to_string()),
				target_port: from_int_or_string_tostring(port.target_port.unwrap_or_default()),
				exposed_port: port.port,
				node_port: port.node_port.unwrap_or_default(),
				protocol: port.protocol.unwrap_or_default(),
			});
		}

		out.push(tmp);
	}

	Ok(())
}
