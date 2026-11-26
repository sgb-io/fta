#![cfg(feature = "use_output")]

use std::fs;
use std::process::Command;

#[test]
fn table_summary_counts_displayed_and_analyzed_files() {
    let project = tempfile::tempdir().unwrap();
    for (file, lines) in [("short.ts", 1), ("medium.ts", 3), ("long.ts", 5)] {
        let source: String = (0..lines)
            .map(|line| format!("export const value{line} = {line};\n"))
            .collect();
        fs::write(project.path().join(file), source).unwrap();
    }
    fs::write(project.path().join("ignored.txt"), "ignored").unwrap();
    fs::write(project.path().join("invalid.ts"), "export const = ;").unwrap();

    for (exclude_under, output_limit, displayed, summary) in [
        ("1", "1", 1, "Showing 1 of 3 files analyzed in "),
        ("1", "3", 2, "Showing 2 of 3 files analyzed in "),
        ("0", "1", 1, "Showing 1 of 3 files analyzed in "),
        ("0", "0", 0, "Showing 0 of 3 files analyzed in "),
        ("0", "3", 3, "3 files analyzed in "),
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_fta"))
            .arg(project.path())
            .args([
                "--exclude-under",
                exclude_under,
                "--output-limit",
                output_limit,
            ])
            .output()
            .unwrap();

        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert_eq!(
            stdout.lines().filter(|line| line.contains(".ts")).count(),
            displayed
        );
        assert!(
            stdout.lines().last().unwrap().starts_with(summary),
            "{stdout}"
        );
    }
}
