use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Category {
    #[serde(rename = "UI")]
    Ui,
    #[serde(rename = "Server")]
    Server,
    #[serde(rename = "Privacy")]
    Privacy,
    #[serde(rename = "About")]
    About,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DefaultValue {
    Bool(bool),
    Str(&'static str),
    Float(f64),
    Int(i64),
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Setting {
    pub id: &'static str,
    pub category: Category,
    pub label: &'static str,
    pub description: &'static str,
    pub default: DefaultValue,
}

pub const CATEGORIES: &[Category] = &[Category::Ui, Category::Server, Category::Privacy, Category::About];

pub const SETTINGS: &[Setting] = &[
    Setting {
        id: "auto-expand",
        category: Category::Ui,
        label: "Auto-expand structures",
        description: "Automatically expand larger data structures",
        default: DefaultValue::Bool(true),
    },
    Setting {
        id: "auto-group-dumps",
        category: Category::Ui,
        label: "Auto group dumps",
        description: "Group multiple variables dumped at once",
        default: DefaultValue::Bool(false),
    },
    Setting {
        id: "long-file-path",
        category: Category::Ui,
        label: "Show full file path",
        description: "Show full path instead of truncated version",
        default: DefaultValue::Bool(false),
    },
    Setting {
        id: "server-port",
        category: Category::Server,
        label: "Listening port",
        description: "Port the Quo server listens on",
        default: DefaultValue::Int(7779),
    },
    Setting {
        id: "analytics",
        category: Category::Privacy,
        label: "Send anonymous analytics",
        description: "Help improve Quo by sending anonymous usage data",
        default: DefaultValue::Bool(false),
    },
];

pub fn settings_for(category: Category) -> impl Iterator<Item = &'static Setting> {
    SETTINGS.iter().filter(move |s| s.category == category)
}
