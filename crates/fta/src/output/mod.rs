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
    let mut output = String::new();

    match Some(format.as_str()) {
        Some("json") => {
            output = serde_json::to_string(file_data_list).unwrap();
        }
        Some("csv") => {
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
        Some("table") => {
            let mut table = Table::new();
            table.load_preset(UTF8_FULL);
            table.set_content_arrangement(comfy_table::ContentArrangement::Dynamic);
            table.set_header(vec![
                "File",
                "Num. lines",
                "FTA Score (Lower is better)",
                "Assessment",
            ]);

            for file_data in file_data_list {
                if table.row_iter().count() >= output_limit {
                    continue;
                }
                table.add_row(vec![
                    truncate_string(&file_data.file_name, 50),
                    file_data.line_count.to_string(),
                    format!("{:.2}", file_data.fta_score),
                    file_data.assessment.clone().to_string(),
                ]);
            }

            output = format!(
                "{}\n{} files analyzed in {}s.",
                table.to_string(),
                file_data_list.len(),
                (elapsed * 10000.0).round() / 10000.0
            );
        }
        _ => output.push_str("No output format specified."),
    }

    output
}
