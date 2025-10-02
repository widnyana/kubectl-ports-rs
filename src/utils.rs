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

use k8s_openapi::{
	api::core::v1::Pod,
	apimachinery::pkg::util::intstr::IntOrString,
};

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

pub fn from_int_or_string_tostring(s: IntOrString) -> String {
	match s {
		IntOrString::Int(i) => i.to_string(),
		IntOrString::String(s) => s,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use k8s_openapi::api::core::v1::{PodCondition, PodStatus};
	use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

	fn create_pod_with_phase(phase: &str) -> Pod {
		Pod {
			metadata: ObjectMeta::default(),
			spec: None,
			status: Some(PodStatus {
				phase: Some(phase.to_string()),
				..Default::default()
			}),
		}
	}

	fn create_pod_with_phase_and_condition(phase: &str, condition_type: &str, status: &str) -> Pod {
		Pod {
			metadata: ObjectMeta::default(),
			spec: None,
			status: Some(PodStatus {
				phase: Some(phase.to_string()),
				conditions: Some(vec![PodCondition {
					type_: condition_type.to_string(),
					status: status.to_string(),
					..Default::default()
				}]),
				..Default::default()
			}),
		}
	}

	#[test]
	fn test_is_scheduled_running_pod() {
		let pod = create_pod_with_phase("Running");
		assert!(is_scheduled(&pod));
	}

	#[test]
	fn test_is_scheduled_succeeded_pod() {
		let pod = create_pod_with_phase("Succeeded");
		assert!(!is_scheduled(&pod));
	}

	#[test]
	fn test_is_scheduled_failed_pod() {
		let pod = create_pod_with_phase("Failed");
		assert!(!is_scheduled(&pod));
	}

	#[test]
	fn test_is_scheduled_unknown_pod() {
		let pod = create_pod_with_phase("Unknown");
		assert!(!is_scheduled(&pod));
	}

	#[test]
	fn test_is_scheduled_pending_scheduled_pod() {
		let pod = create_pod_with_phase_and_condition("Pending", "PodScheduled", "True");
		assert!(is_scheduled(&pod));
	}

	#[test]
	fn test_is_scheduled_pending_unscheduled_pod() {
		let pod = create_pod_with_phase_and_condition("Pending", "PodScheduled", "False");
		assert!(!is_scheduled(&pod));
	}

	#[test]
	fn test_is_scheduled_pending_no_conditions() {
		let pod = create_pod_with_phase("Pending");
		assert!(!is_scheduled(&pod));
	}

	#[test]
	fn test_is_scheduled_no_status() {
		let pod = Pod {
			metadata: ObjectMeta::default(),
			spec: None,
			status: None,
		};
		assert!(!is_scheduled(&pod));
	}

	#[test]
	fn test_is_scheduled_no_phase() {
		let pod = Pod {
			metadata: ObjectMeta::default(),
			spec: None,
			status: Some(PodStatus {
				phase: None,
				..Default::default()
			}),
		};
		assert!(!is_scheduled(&pod));
	}

	#[test]
	fn test_from_int_or_string_with_int() {
		let value = IntOrString::Int(8080);
		assert_eq!(from_int_or_string_tostring(value), "8080");
	}

	#[test]
	fn test_from_int_or_string_with_string() {
		let value = IntOrString::String("http".to_string());
		assert_eq!(from_int_or_string_tostring(value), "http");
	}

	#[test]
	fn test_from_int_or_string_with_zero() {
		let value = IntOrString::Int(0);
		assert_eq!(from_int_or_string_tostring(value), "0");
	}

	#[test]
	fn test_from_int_or_string_with_negative() {
		let value = IntOrString::Int(-1);
		assert_eq!(from_int_or_string_tostring(value), "-1");
	}

	#[test]
	fn test_from_int_or_string_with_empty_string() {
		let value = IntOrString::String("".to_string());
		assert_eq!(from_int_or_string_tostring(value), "");
	}
}
