use crate::structs::FileData;
use comfy_table::{presets::UTF8_FULL, Table};

mod tests;

pub fn truncate_string(input: &str, max_length: usize) -> String {
    if input.len() <= max_length {
        input.to_string()
    } else {
        format!("...{}", &input[input.len() - max_length + 3..])
    }
}

pub fn generate_output(
    file_data_list: &Vec<FileData>,
    format: String,
    elapsed: &f64,
    output_limit: usize,
) -> String {
    generate_output_with_count(
        file_data_list,
        file_data_list.len(),
        &format,
        *elapsed,
        output_limit,
    )
}

/// Formats results using the number of files successfully analyzed before filtering.
pub fn generate_output_with_count(
    file_data_list: &[FileData],
    analyzed_count: usize,
    format: &str,
    elapsed: f64,
    output_limit: usize,
) -> String {
    let mut output = String::new();

    match format {
        "json" => {
            output = serde_json::to_string(file_data_list).unwrap();
        }
        "csv" => {
            output.push_str("File,Num. lines,FTA Score (Lower is better),Assessment");
            for file_data in file_data_list {
                output.push_str(&format!(
                    "\n{},{},{:.2},{}",
                    file_data.file_name,
                    file_data.line_count,
                    file_data.fta_score,
                    file_data.assessment
                ));
            }
        }
        "table" => {
            let mut table = Table::new();
            table.load_preset(UTF8_FULL);
            table.set_content_arrangement(comfy_table::ContentArrangement::Dynamic);
            table.force_no_tty();
            table.set_width(80);
            table.set_header(vec![
                "File",
                "Num. lines",
                "FTA Score (Lower is better)",
                "Assessment",
            ]);

            for file_data in file_data_list.iter().take(output_limit) {
                table.add_row(vec![
                    truncate_string(&file_data.file_name, 50),
                    file_data.line_count.to_string(),
                    format!("{:.2}", file_data.fta_score),
                    file_data.assessment.clone().to_string(),
                ]);
            }

            let shown = file_data_list.len().min(output_limit);
            let count = if shown < analyzed_count {
                format!("Showing {shown} of {analyzed_count}")
            } else {
                analyzed_count.to_string()
            };
            let elapsed = (elapsed * 10000.0).round() / 10000.0;
            output = format!("{table}\n{count} files analyzed in {elapsed}s.");
        }
        _ => output.push_str("No output format specified."),
    }

    output
}
