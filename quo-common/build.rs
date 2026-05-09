use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("available_themes.rs");

    let mut themes = Vec::new();

    let styles_path = Path::new("../src/styles");

    if let Ok(entries) = fs::read_dir(styles_path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    if let Some(name) = entry.file_name().to_str() {
                        themes.push(name.to_string());
                    }
                }
            }
        }
    }

    if themes.is_empty() {
        themes.push("Quo (default)".to_string());
    }

    themes.sort();
    if let Some(pos) = themes.iter().position(|t| t == "Quo (default)") {
        let quo_default = themes.remove(pos);
        themes.insert(0, quo_default);
    }

    let themes_code = format!(
        "pub const AVAILABLE_THEMES: &[&str] = &[{}];",
        themes.iter().map(|t| format!("{:?}", t)).collect::<Vec<_>>().join(", ")
    );

    fs::write(dest_path, themes_code).unwrap();
    println!("cargo:rerun-if-changed=../src/styles");
}
