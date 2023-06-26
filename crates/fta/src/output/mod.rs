mod tests;

use crate::structs::FileData;
use comfy_table::presets::UTF8_FULL;
use comfy_table::Table;

fn truncate_string(input: &str, max_length: usize) -> String {
    if input.len() <= max_length {
        input.to_string()
    } else {
        format!("...{}", &input[input.len() - max_length + 3..])
    }
}

pub fn output(file_data_list: &Vec<FileData>, format: String, elapsed: &f64) {
    match Some(format.as_str()) {
        Some("json") => {
            let json_string = serde_json::to_string(file_data_list).unwrap();
            println!("{}", json_string);
        }
        Some("csv") => {
            println!("File,Num. lines,FTA Score (Lower is better),Assessment");
            for file_data in file_data_list {
                println!(
                    "{},{},{:.2},{}",
                    file_data.file_name,
                    file_data.line_count,
                    file_data.fta_score,
                    file_data.assessment
                );
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
                table.add_row(vec![
                    truncate_string(&file_data.file_name, 50),
                    file_data.line_count.to_string(),
                    format!("{:.2}", file_data.fta_score),
                    file_data.assessment.clone().to_string(),
                ]);
            }

            println!("{table}");

            println!(
                "{} files analyzed in {}s.",
                file_data_list.len(),
                (elapsed * 10000.0).round() / 10000.0
            );
        }
        _ => {
            println!("No output format specified.");
        }
    }
}
