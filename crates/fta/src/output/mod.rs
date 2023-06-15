mod tests;

use crate::structs::FileData;
use comfy_table::Table;

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
            table.set_header(vec![
                "File",
                "Num. lines",
                "FTA Score (Lower is better)",
                "Assessment",
            ]);

            for file_data in file_data_list {
                table.add_row(vec![
                    file_data.file_name.clone().to_string(),
                    file_data.line_count.to_string(),
                    file_data.fta_score.to_string(),
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
