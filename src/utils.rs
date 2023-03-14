// The phase of a Pod is a simple, high-level summary of where the Pod is in its
// lifecycle. The conditions array, the reason and message fields, and the
// individual container status arrays contain more detail about the pod's
// status.
//
// There are five possible phase values:
// Pending: The pod has been accepted by the Kubernetes system, but one or more
// of the container images has not been created. This includes time before being
// scheduled as well as time spent downloading images over the network, which
// could take a while. Running: The pod has been bound to a node, and all of the
// containers have been created. At least one container is still running, or is
// in the process of starting or restarting. Succeeded: All containers in the
// pod have terminated in success, and will not be restarted. Failed: All
// containers in the pod have terminated, and at least one container has
// terminated in failure. The container either exited with non-zero status or
// was terminated by the system. Unknown: For some reason the state of the pod
// could not be obtained, typically due to an error in communicating with the
// host of the pod.
//
// More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-phase

use k8s_openapi::api::core::v1::Pod;

#[allow(clippy::match_same_arms)]
pub fn is_scheduled(pod: &Pod) -> bool {
	pod.status
		.as_ref()
		.and_then(|ps| {
			ps.phase.as_ref().and_then(|phase| {
				match &phase[..] {
					"Succeeded" | "Failed" => Some(false),
					"Running" => Some(true),
					"Unknown" => None, // this is the case when a node is down (kubelet is not
					// responding)
					"Pending" => ps
						.conditions
						.as_ref()
						.map(|o| o.iter().any(|c| c.type_ == "PodScheduled" && c.status == "True")),
					&_ => None, // should not happen
				}
			})
		})
		.unwrap_or(false)
}
