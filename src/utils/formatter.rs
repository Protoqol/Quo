/// NOTE: these code formatters were written with the help of an AI assistant.
use quo_common::payloads::{IncomingQuoPayload, QuoPayloadLanguage};

pub fn format_by_language(dump: &IncomingQuoPayload) -> String {
    match dump.language {
        QuoPayloadLanguage::Rust => format_rust(dump),
        QuoPayloadLanguage::Typescript | QuoPayloadLanguage::Javascript => {
            format_javascript_typescript(dump)
        }
        QuoPayloadLanguage::Php => format_php(dump),
        QuoPayloadLanguage::Go => format_go(dump),
        _ => format_generic(dump),
    }
}

fn format_rust(dump: &IncomingQuoPayload) -> String {
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
        "${} = {} // {}",
        dump.meta.variable.name,
        format_code_snippet(&dump.meta.variable.value, 4),
        dump.meta.variable.var_type,
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
                indent_level = indent_level.saturating_sub(1);

                let suffix = format!("\n{}", indent.repeat(indent_level + 1));

                if formatted.ends_with(&suffix) {
                    formatted.truncate(formatted.len() - suffix.len());
                } else {
                    formatted.push('\n');
                    formatted.push_str(&indent.repeat(indent_level));
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
