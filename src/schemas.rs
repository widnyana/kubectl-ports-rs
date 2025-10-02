#[derive(Debug, Clone)]
pub struct ContainerPort {
	pub container_name: String,

	pub container_port: i32,

	/// If specified, this must be an IANA_SVC_NAME and unique within the pod.
	/// Each named port in a pod must have a unique name. Name for the port that
	/// can be referred to by services.
	pub name: String,

	/// Protocol for port. Must be UDP, TCP, or SCTP. Defaults to "TCP".
	pub protocol: String,
}

#[derive(Debug, Clone)]
pub struct Resource {
	pub kind: String,
	pub name: String,
	pub namespace: String,
	pub ports: Vec<ContainerPort>,
}

impl Resource {
	pub fn new(kind: String, name: String, namespace: String) -> Resource {
		Resource { kind, name, namespace, ports: vec![] }
	}
}

///
#[derive(Debug, Clone)]
pub struct ServiceResource {
	pub name: String,
	pub namespace: String,
	pub svc_type: String,
	pub cluster_ip: String,
	pub external_traffic_policy: String,
	pub ports: Vec<ServicePort>,
}

impl ServiceResource {
	pub fn new(
		name: String,
		ns: String,
		svc_type: String,
		cluster_ip: String,
		external_traffic_policy: String,
	) -> ServiceResource {
		ServiceResource {
			name,
			namespace: ns,
			svc_type,
			cluster_ip,
			external_traffic_policy,
			ports: vec![],
		}
	}
}

#[derive(Debug, Clone)]
pub struct ServicePort {
	pub port_name: String,
	pub target_port: String,
	pub exposed_port: i32,
	pub node_port: i32,
	pub protocol: String,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_resource_new() {
		let resource = Resource::new(
			"pod".to_string(),
			"test-pod".to_string(),
			"default".to_string(),
		);

		assert_eq!(resource.kind, "pod");
		assert_eq!(resource.name, "test-pod");
		assert_eq!(resource.namespace, "default");
		assert_eq!(resource.ports.len(), 0);
	}

	#[test]
	fn test_resource_with_ports() {
		let mut resource = Resource::new(
			"pod".to_string(),
			"test-pod".to_string(),
			"default".to_string(),
		);

		let port = ContainerPort {
			container_name: "nginx".to_string(),
			container_port: 8080,
			name: "http".to_string(),
			protocol: "TCP".to_string(),
		};

		resource.ports.push(port);
		assert_eq!(resource.ports.len(), 1);
		assert_eq!(resource.ports[0].container_name, "nginx");
		assert_eq!(resource.ports[0].container_port, 8080);
	}

	#[test]
	fn test_service_resource_new() {
		let svc = ServiceResource::new(
			"test-service".to_string(),
			"default".to_string(),
			"ClusterIP".to_string(),
			"10.0.0.1".to_string(),
			"Cluster".to_string(),
		);

		assert_eq!(svc.name, "test-service");
		assert_eq!(svc.namespace, "default");
		assert_eq!(svc.svc_type, "ClusterIP");
		assert_eq!(svc.cluster_ip, "10.0.0.1");
		assert_eq!(svc.external_traffic_policy, "Cluster");
		assert_eq!(svc.ports.len(), 0);
	}

	#[test]
	fn test_service_resource_with_ports() {
		let mut svc = ServiceResource::new(
			"test-service".to_string(),
			"default".to_string(),
			"NodePort".to_string(),
			"10.0.0.1".to_string(),
			"Cluster".to_string(),
		);

		let port = ServicePort {
			port_name: "http".to_string(),
			target_port: "8080".to_string(),
			exposed_port: 80,
			node_port: 30080,
			protocol: "TCP".to_string(),
		};

		svc.ports.push(port);
		assert_eq!(svc.ports.len(), 1);
		assert_eq!(svc.ports[0].port_name, "http");
		assert_eq!(svc.ports[0].exposed_port, 80);
		assert_eq!(svc.ports[0].node_port, 30080);
	}

	#[test]
	fn test_container_port_creation() {
		let port = ContainerPort {
			container_name: "app".to_string(),
			container_port: 3000,
			name: "api".to_string(),
			protocol: "TCP".to_string(),
		};

		assert_eq!(port.container_name, "app");
		assert_eq!(port.container_port, 3000);
		assert_eq!(port.name, "api");
		assert_eq!(port.protocol, "TCP");
	}

	#[test]
	fn test_service_port_creation() {
		let port = ServicePort {
			port_name: "https".to_string(),
			target_port: "443".to_string(),
			exposed_port: 443,
			node_port: 30443,
			protocol: "TCP".to_string(),
		};

		assert_eq!(port.port_name, "https");
		assert_eq!(port.target_port, "443");
		assert_eq!(port.exposed_port, 443);
		assert_eq!(port.node_port, 30443);
		assert_eq!(port.protocol, "TCP");
	}
}
