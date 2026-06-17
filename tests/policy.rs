#[test]
fn test_no_dependencies() {
    let cargo_toml = include_str!("../Cargo.toml");
    let has_dependencies = cargo_toml.lines().any(|line| {
        let trimmed = line.trim();
        trimmed == "[dependencies]" || trimmed.starts_with("[dependencies.")
    });

    if has_dependencies {
        let lines: Vec<&str> = cargo_toml.lines().collect();
        for i in 0..lines.len() {
            if lines[i].trim() == "[dependencies]" {
                for next_line in lines.iter().skip(i + 1) {
                    let next_line = next_line.trim();
                    if next_line.is_empty() {
                        continue;
                    }
                    if next_line.starts_with('[') {
                        break;
                    }
                    // Allow optional dependencies or internal macro workspace members
                    if !next_line.contains("optional = true")
                        && !next_line.starts_with("rustand-macros")
                    {
                        panic!(
                            "Non-optional external dependencies are not allowed: {}",
                            next_line
                        );
                    }
                }
            }
        }
    }
}
