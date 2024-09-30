#[cfg(test)]
mod tests {
    use afmt::config::*;
    use colored::Colorize;
    use similar::{ChangeTag, TextDiff};
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use std::process::Command;

    #[test]
    fn manual() {
        let (total, failed) = run_scenario("tests/static", "static");
        assert_eq!(failed, 0, "{} out of {} tests failed", failed, total);
    }

    #[test]
    fn prettier() {
        let (total, failed) = run_scenario("tests/prettier", "prettier");
        assert_eq!(failed, 0, "{} out of {} tests failed", failed, total);
    }

    #[test]
    fn extra() {
        let (total, failed) = run_scenario("tests/prettier2", "prettier2");
        assert_eq!(failed, 0, "{} out of {} tests failed", failed, total);
    }

    #[test]
    fn all() {
        let scenarios = [
            ("tests/static", "static"),
            ("tests/prettier", "prettier"),
            ("tests/prettier2", "prettier2"),
        ];

        let mut total_tests = 0;
        let mut failed_tests = 0;

        println!("Running all test scenarios...");

        for (path, name) in scenarios.iter() {
            let (tests, failures) = run_scenario(path, name);
            total_tests += tests;
            failed_tests += failures;
        }

        println!(
            "\nTest Summary: {}/{} tests passed",
            total_tests - failed_tests,
            total_tests
        );
        assert_eq!(
            failed_tests, 0,
            "{} out of {} tests failed",
            failed_tests, total_tests
        );
    }

    fn run_scenario(dir_path: &str, scenario_name: &str) -> (u32, u32) {
        let mut total_tests = 0;
        let mut failed_tests = 0;

        for entry in std::fs::read_dir(dir_path).unwrap() {
            let entry = entry.unwrap();
            let source = entry.path();
            if source.extension().and_then(|ext| ext.to_str()) == Some("in") {
                println!(
                    "{} {:?}",
                    format!("### Processing {} file:", scenario_name).green(),
                    source
                );
                total_tests += 1;
                if !run_test_file(&source, scenario_name) {
                    failed_tests += 1;
                }
            }
        }

        println!(
            "{} scenario: {}/{} tests passed",
            scenario_name,
            total_tests - failed_tests,
            total_tests
        );
        (total_tests, failed_tests)
    }

    fn run_test_file(source: &Path, scenario_name: &str) -> bool {
        match scenario_name {
            "static" => run_static_test_files(source),
            "prettier" | "prettier2" => run_prettier_test_files(source),
            _ => panic!("Unknown scenario: {}", scenario_name),
        }
    }

    fn run_static_test_files(source: &Path) -> bool {
        let expected_file = source.with_extension("cls");
        let output = format_with_afmt(source);
        let expected =
            std::fs::read_to_string(expected_file).expect("Failed to read expected .cls file");

        compare("Static:", output, expected, source)
    }

    fn run_prettier_test_files(source: &Path) -> bool {
        let prettier_file = source.with_extension("cls");

        if !prettier_file.exists() {
            println!("{}", "### .cls file not found, generating...".yellow());
            let prettier_output = run_prettier(source).expect("Failed to run Prettier");
            save_prettier_output(&prettier_file, &prettier_output);
        }

        let output = format_with_afmt(source);
        let prettier_output =
            std::fs::read_to_string(&prettier_file).expect("Failed to read the .cls file.");

        compare("Prettier:", output, prettier_output, source)
    }

    fn compare(against: &str, output: String, expected: String, source: &Path) -> bool {
        if output != expected {
            let source_content =
                std::fs::read_to_string(source).expect("Failed to read the file content.");

            println!("\nFailed: {:?}:", source);
            println!("-------------------------------------\n");
            println!("{}", source_content);
            println!("-------------------------------------\n");
            print_side_by_side_diff(against, &output, &expected);
            println!("\n-------------------------------------\n");

            false
        } else {
            true
        }
    }

    fn format_with_afmt(source: &Path) -> String {
        let file_path = source
            .to_str()
            .expect("PathBuf to String failed.")
            .to_string();
        let session = Session::new(Config::default(), vec![file_path.clone()]);
        let vec = session.format();
        vec.into_iter()
            .next()
            .and_then(|result| result.ok())
            .expect("format result failed.")
    }

    fn print_side_by_side_diff(against: &str, output: &str, expected: &str) {
        let diff = TextDiff::from_lines(expected, output);
        let mut left_col;
        let mut right_col;
        println!(
            "\x1b[38;2;255;165;0m{:<60} | {:<60}\x1b[0m",
            against, "Afmt:\n"
        );
        for change in diff.iter_all_changes() {
            match change.tag() {
                ChangeTag::Delete => {
                    left_col = format!("\x1b[91m- {:<58}\x1b[0m", change.to_string().trim_end()); // Red for deletions (left)
                    right_col = String::from(""); // Empty on the right side
                }
                ChangeTag::Insert => {
                    left_col = String::from(""); // Empty on the left side
                    right_col = format!("\x1b[92m+ {:<58}\x1b[0m", change.to_string().trim_end());
                    // Green for insertions (right)
                }
                ChangeTag::Equal => {
                    left_col = format!("  {:<58}", change.to_string().trim_end()); // No color for unchanged lines
                    right_col = format!("  {:<58}", change.to_string().trim_end());
                }
            }
            println!("{:<60} | {:<60}", left_col, right_col);
        }
    }

    fn run_prettier(source: &Path) -> Result<String, String> {
        let output = Command::new("npx")
            .arg("prettier")
            .arg("--plugin=prettier-plugin-apex")
            .arg("--parser=apex")
            .arg(source.to_str().unwrap())
            .output()
            .expect("Failed to execute Prettier");

        if output.status.success() {
            let formatted_code =
                String::from_utf8(output.stdout).expect("Prettier output is not valid UTF-8");
            Ok(formatted_code)
        } else {
            let error_message = String::from_utf8(output.stderr)
                .unwrap_or_else(|_| "Unknown error while running Prettier".to_string());
            Err(error_message)
        }
    }

    fn save_prettier_output(file_path: &Path, output: &str) {
        let mut file = File::create(file_path).expect("Failed to create .cls file");
        file.write_all(output.as_bytes())
            .expect("Failed to write Prettier output");
    }
}
