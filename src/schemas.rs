#[derive(Debug, Clone)]
pub struct ContainerPort {
	pub container_name: String,

	pub container_port: i32,

	/// What host IP to bind the external port to.
	pub host_ip: String,

	/// Number of port to expose on the host. If specified, this must be a valid
	/// port number, 0 \< x \< 65536. If HostNetwork is specified, this must
	/// match ContainerPort. Most containers do not need this.
	pub host_port: i32,

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
