use similar::{ChangeTag, TextDiff};

#[tauri::command]
pub fn get_diff_for_snippets(first: String, second: String) -> String {
    let mut diffed = String::new();
    let to_diff = TextDiff::from_lines(&first, &second);

    for change in to_diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "-",
            ChangeTag::Insert => "+",
            ChangeTag::Equal => " ",
        };

        let formatted_diff = format!("{}{}", sign, change);

        diffed.push_str(&formatted_diff);
    }

    diffed
}
