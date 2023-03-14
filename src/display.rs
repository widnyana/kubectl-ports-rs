use prettytable::{
	cell,
	format,
	row,
	Row,
	Table,
};

use crate::schemas::Resource;

pub async fn render_pod_as_table(data: &Vec<Resource>) {
	let mut table = Table::new();
	let format = format::FormatBuilder::new()
		.separators(
			&[
				format::LinePosition::Top,
				format::LinePosition::Bottom,
				format::LinePosition::Title,
			],
			format::LineSeparator::new('-', '+', '+', '+'),
		)
		.column_separator('|')
		.borders('|')
		.padding(1, 1)
		.build();
	table.set_format(format);

	let row_titles = row![
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
