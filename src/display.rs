use prettytable::{
	cell,
	format,
	format::TableFormat,
	row,
	Row,
	Table,
};

use crate::schemas::{
	Resource,
	ServiceResource,
};

fn get_table_format() -> TableFormat {
	format::FormatBuilder::new()
		.separators(
			&[
				format::LinePosition::Top,
				format::LinePosition::Bottom,
				format::LinePosition::Title,
			],
			format::LineSeparator::new('=', '+', '+', '+'),
		)
		.separator(format::LinePosition::Intern, format::LineSeparator::new('-', '+', '+', '+'))
		.column_separator('|')
		.borders('|')
		.padding(2, 2)
		.indent(2)
		.build()
}

pub async fn render_pod_as_table(data: &Vec<Resource>) {
	let mut table = Table::new();
	table.set_format(get_table_format());

	let row_titles = row![
		bc->"Kind",
		bc->"Namespace",
		bc->"Pod Name",
		bc->"Container name",
		bc->"Container Port",
		bc->"Port Name",
	];
	table.set_titles(row_titles);

	for d in data {
		for p in d.ports.clone() {
			#[allow(clippy::string_to_string)]
			let c = vec![
				cell!(d.kind),
				cell!(d.namespace),
				cell!(d.name),
				cell!(p.container_name.clone()),
				cell!(format!("{}/{}", p.container_port, p.protocol)),
				cell!(p.name),
			];
			table.add_row(Row::new(c));
		}
	}

	table.printstd();
}

pub async fn render_svc_as_table(svcs: &Vec<ServiceResource>) {
	let mut table = Table::new();
	table.set_format(get_table_format());

	let row_titles = row![
		bc->"Namespace",
		bc->"Service Name",
		bc->"Type",
		bc->"Cluster IP",
		bc->"Port Name",
		bc->"Target Port",
		bc->"Exposed Port",
		bc->"Node Port",
		bc->"External Traffic Policy",
	];
	table.set_titles(row_titles);

	for svc in svcs {
		for p in svc.ports.clone() {
			#[allow(clippy::string_to_string)]
			let c = vec![
				cell!(svc.namespace),
				cell!(svc.name),
				cell!(svc.svc_type),
				cell!(svc.cluster_ip),
				cell!(c->p.port_name),
				cell!(format!("{}/{}", p.target_port, p.protocol)),
				cell!(format!("{}/{}", p.exposed_port, p.protocol)),
				cell!(format!("{}/{}", p.node_port, p.protocol)),
				cell!(svc.external_traffic_policy),
			];
			table.add_row(Row::new(c));
		}
	}

	table.printstd();
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::schemas::{ContainerPort, ServicePort};

	#[test]
	fn test_get_table_format() {
		let format = get_table_format();
		assert!(std::mem::size_of_val(&format) > 0);
	}

	#[test]
	fn test_render_pod_as_table_with_empty_data() {
		let data = vec![];
		let _ = tokio::runtime::Runtime::new()
			.unwrap()
			.block_on(render_pod_as_table(&data));
	}

	#[test]
	fn test_render_pod_as_table_with_single_resource() {
		let mut resource = Resource::new(
			"pod".to_string(),
			"test-pod".to_string(),
			"default".to_string(),
		);

		resource.ports.push(ContainerPort {
			container_name: "nginx".to_string(),
			container_port: 80,
			name: "http".to_string(),
			protocol: "TCP".to_string(),
		});

		let data = vec![resource];
		let _ = tokio::runtime::Runtime::new()
			.unwrap()
			.block_on(render_pod_as_table(&data));
	}

	#[test]
	fn test_render_svc_as_table_with_empty_data() {
		let data = vec![];
		let _ = tokio::runtime::Runtime::new()
			.unwrap()
			.block_on(render_svc_as_table(&data));
	}

	#[test]
	fn test_render_svc_as_table_with_single_service() {
		let mut svc = ServiceResource::new(
			"test-service".to_string(),
			"default".to_string(),
			"ClusterIP".to_string(),
			"10.0.0.1".to_string(),
			"Cluster".to_string(),
		);

		svc.ports.push(ServicePort {
			port_name: "http".to_string(),
			target_port: "8080".to_string(),
			exposed_port: 80,
			node_port: 0,
			protocol: "TCP".to_string(),
		});

		let data = vec![svc];
		let _ = tokio::runtime::Runtime::new()
			.unwrap()
			.block_on(render_svc_as_table(&data));
	}
}
