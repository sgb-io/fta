use crate::structs::FileData;
use comfy_table::{presets::UTF8_FULL, Cell, Color, Table};

mod tests;

/// Map FTA score to appropriate color for display
#[cfg(feature = "use_output")]
fn get_score_color(score: f64) -> Color {
    if score > 60.0 {
        Color::Red
    } else if score > 50.0 {
        Color::Yellow
    } else {
        Color::Green
    }
}

/// Map assessment to appropriate color for display
#[cfg(feature = "use_output")]
fn get_assessment_color(assessment: &str) -> Color {
    match assessment {
        "Needs improvement" => Color::Red,
        "Could be better" => Color::Yellow,
        "OK" => Color::Green,
        _ => Color::White,
    }
}

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
    generate_output_with_colors(file_data_list, format, elapsed, output_limit, true)
}

pub fn generate_output_with_colors(
    file_data_list: &Vec<FileData>,
    format: String,
    elapsed: &f64,
    output_limit: usize,
    use_colors: bool,
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
            // Remove force_no_tty() to allow colors
            table.set_width(80);

            // Set header with or without colors
            if use_colors {
                #[cfg(feature = "use_output")]
                {
                    table.set_header(vec![
                        Cell::new("File").fg(Color::White),
                        Cell::new("Num. lines").fg(Color::White),
                        Cell::new("FTA Score (Lower is better)").fg(Color::White),
                        Cell::new("Assessment").fg(Color::White),
                    ]);
                }
                #[cfg(not(feature = "use_output"))]
                {
                    table.set_header(vec![
                        "File",
                        "Num. lines",
                        "FTA Score (Lower is better)",
                        "Assessment",
                    ]);
                }
            } else {
                table.set_header(vec![
                    "File",
                    "Num. lines",
                    "FTA Score (Lower is better)",
                    "Assessment",
                ]);
            }

            for file_data in file_data_list {
                if table.row_iter().count() >= output_limit {
                    continue;
                }

                if use_colors {
                    #[cfg(feature = "use_output")]
                    {
                        let score_color = get_score_color(file_data.fta_score);
                        let assessment_color = get_assessment_color(&file_data.assessment);

                        table.add_row(vec![
                            Cell::new(truncate_string(&file_data.file_name, 50))
                                .fg(Color::DarkGrey),
                            Cell::new(file_data.line_count.to_string()).fg(Color::DarkGrey),
                            Cell::new(format!("{:.2}", file_data.fta_score)).fg(score_color),
                            Cell::new(file_data.assessment.clone().to_string())
                                .fg(assessment_color),
                        ]);
                    }
                    #[cfg(not(feature = "use_output"))]
                    {
                        table.add_row(vec![
                            truncate_string(&file_data.file_name, 50),
                            file_data.line_count.to_string(),
                            format!("{:.2}", file_data.fta_score),
                            file_data.assessment.clone().to_string(),
                        ]);
                    }
                } else {
                    table.add_row(vec![
                        truncate_string(&file_data.file_name, 50),
                        file_data.line_count.to_string(),
                        format!("{:.2}", file_data.fta_score),
                        file_data.assessment.clone().to_string(),
                    ]);
                }
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
