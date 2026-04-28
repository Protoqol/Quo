use quo_common::payloads::{IncomingQuoPayload, QuoPayloadLanguage};
use rust_format::{Formatter, RustFmt};

pub fn format_by_language(dump: &IncomingQuoPayload, truncate: bool) -> String {
    let mut dump = dump.clone();

    if truncate {
        dump.meta.variable.var_type = truncate_type(&dump.meta.variable.var_type);
    }

    match dump.language {
        QuoPayloadLanguage::Rust => format_rust(&dump),
        QuoPayloadLanguage::Typescript | QuoPayloadLanguage::Javascript => {
            format_javascript_typescript(&dump)
        }
        QuoPayloadLanguage::Php => format_php(&dump),
        QuoPayloadLanguage::Go => format_go(&dump),
        _ => format_generic(&dump),
    }
}

fn truncate_type(var_type: &str) -> String {
    if var_type.contains("::") {
        let parts: Vec<&str> = var_type.split("::").collect();
        if parts.len() > 2 {
            return format!("{}::...::{}", parts[0], parts.last().unwrap());
        }
    }

    if var_type.contains('\\') {
        let parts: Vec<&str> = var_type.split('\\').collect();
        if parts.len() > 2 {
            return format!("{}\\...\\{}", parts[0], parts.last().unwrap());
        }
    }

    var_type.to_string()
}

fn format_rust(dump: &IncomingQuoPayload) -> String {
    fn format_expression(dump: &IncomingQuoPayload) -> String {
        if dump.meta.variable.name == dump.meta.variable.value {
            return format!(
                "{}: {}",
                dump.meta.variable.name, dump.meta.variable.var_type
            );
        }

        format!(
            "{}: {} = {}",
            dump.meta.variable.name,
            dump.meta.variable.var_type,
            format_code_snippet(&dump.meta.variable.value, 4),
        )
    }

    fn format_variable(dump: &IncomingQuoPayload) -> String {
        format!(
            "{} {}: {} = {}",
            if dump.meta.variable.is_constant {
                "const"
            } else {
                "let"
            },
            dump.meta.variable.name,
            dump.meta.variable.var_type,
            format_code_snippet(&dump.meta.variable.value, 4),
        )
    }

    let declaration: String = match dump.meta.variable.is_expression {
        true => format_expression(dump),
        false => format_variable(dump),
    };

    let to_format = format!("fn main() {{ {}; }}", declaration);

    match RustFmt::default().format_str(&to_format) {
        Ok(formatted) => {
            let trimmed = formatted.trim();
            if let Some(start) = trimmed.find('{') {
                if let Some(end) = trimmed.rfind('}') {
                    let content = &trimmed[start + 1..end];
                    let lines: Vec<&str> = content.lines().collect();
                    if lines.is_empty() {
                        return String::new();
                    }

                    // Determine common indentation to strip
                    let mut min_indent = usize::MAX;
                    for line in lines.iter().skip(1) {
                        let trimmed = line.trim_end();
                        if trimmed.is_empty() {
                            continue;
                        }
                        let indent = trimmed.chars().take_while(|c| c.is_whitespace()).count();
                        if indent < min_indent {
                            min_indent = indent;
                        }
                    }

                    if min_indent == usize::MAX {
                        min_indent = 0;
                    }

                    let mut result = String::new();
                    for (i, line) in lines.iter().enumerate() {
                        let line = line.trim_end();
                        let trimmed_line = if i == 0 {
                            line.trim_start()
                        } else if line.len() >= min_indent {
                            &line[min_indent..]
                        } else {
                            line.trim_start()
                        };
                        result.push_str(trimmed_line);
                        if i < lines.len() - 1 {
                            result.push('\n');
                        }
                    }

                    let result = result.trim();
                    if result.ends_with(';') {
                        return result[..result.len() - 1].trim().to_string();
                    }
                    return result.to_string();
                }
            }
            formatted.to_string()
        }
        Err(err) => {
            eprintln!("RustFmt error {}", err);

            // Fallback to generic formatting if RustFmt fails
            match dump.meta.variable.is_expression {
                true => format_expression(dump),
                false => format_variable(dump),
            }
        }
    }
}

fn format_javascript_typescript(dump: &IncomingQuoPayload) -> String {
    let declaration = format!(
        "{} {}: {}",
        if dump.meta.variable.is_constant {
            "const"
        } else {
            "let"
        },
        dump.meta.variable.name,
        dump.meta.variable.var_type,
    );

    format!(
        "{} = {}",
        declaration,
        format_code_snippet(&dump.meta.variable.value, 4)
    )
}

fn format_php(dump: &IncomingQuoPayload) -> String {
    // @TODO find better way display type UI wise
    format!(
        "// @var {}\n{}{} = {}",
        dump.meta.variable.var_type,
        if dump.meta.variable.is_constant {
            "const "
        } else {
            "$"
        },
        dump.meta.variable.name,
        format_code_snippet(&dump.meta.variable.value, 4),
    )
}

fn format_go(dump: &IncomingQuoPayload) -> String {
    format!(
        "var {} {} = {}",
        dump.meta.variable.name,
        dump.meta.variable.var_type,
        format_code_snippet(&dump.meta.variable.value, 4)
    )
}

fn format_generic(dump: &IncomingQuoPayload) -> String {
    let declaration = format!(
        "{} {}: {}",
        if dump.meta.variable.is_constant {
            "const"
        } else {
            "let"
        },
        dump.meta.variable.name,
        dump.meta.variable.var_type,
    );

    format!(
        "{} = {}",
        declaration,
        format_code_snippet(&dump.meta.variable.value, 4)
    )
}

fn format_code_snippet(code: &str, indent_size: usize) -> String {
    let mut formatted = String::new();
    let mut indent_level = 0;
    let mut in_string = false;
    let mut string_char = ' ';
    let mut is_escaped = false;
    let mut chars = code.chars().peekable();

    let indent = " ".repeat(indent_size);

    while let Some(c) = chars.next() {
        if in_string {
            formatted.push(c);
            if is_escaped {
                is_escaped = false;
            } else if c == '\\' {
                is_escaped = true;
            } else if c == string_char {
                in_string = false;
            }
            continue;
        }

        match c {
            '"' | '\'' => {
                in_string = true;
                string_char = c;
                formatted.push(c);
            }
            '{' | '[' | '(' => {
                let closing = match c {
                    '{' => '}',
                    '[' => ']',
                    '(' => ')',
                    _ => unreachable!(),
                };

                formatted.push(c);
                if chars.peek() == Some(&closing) {
                    formatted.push(chars.next().unwrap());
                } else {
                    indent_level += 1;
                    formatted.push('\n');
                    formatted.push_str(&indent.repeat(indent_level));
                }
            }
            '}' | ']' | ')' => {
                let current_indent = indent_size * indent_level;
                indent_level = indent_level.saturating_sub(1);
                let new_indent = indent_size * indent_level;

                let suffix = format!("\n{}", " ".repeat(current_indent));

                if formatted.ends_with(&suffix) {
                    formatted.truncate(formatted.len() - suffix.len());
                } else {
                    formatted.push('\n');
                    formatted.push_str(&" ".repeat(new_indent));
                }
                formatted.push(c);
            }
            ',' => {
                formatted.push(c);
                formatted.push('\n');
                formatted.push_str(&indent.repeat(indent_level));

                while let Some(&next_c) = chars.peek() {
                    if next_c.is_whitespace() {
                        chars.next();
                    } else {
                        break;
                    }
                }
            }
            _ if c.is_whitespace() => {
                if !formatted.is_empty() && !formatted.ends_with('\n') && !formatted.ends_with(' ')
                {
                    formatted.push(c);
                }
            }
            _ => {
                formatted.push(c);
            }
        }
    }

    formatted.trim().to_string()
}
