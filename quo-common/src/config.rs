use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Category {
    #[serde(rename = "Quo")]
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
    /// Whether this setting should appear in the sidebar quick-settings panel.
    pub show_in_sidebar: bool,
}

pub const CATEGORIES: &[Category] = &[Category::Ui, Category::Server, Category::Privacy, Category::About];

pub const SETTINGS: &[Setting] = &[
    Setting {
        id: "auto-expand",
        category: Category::Ui,
        label: "Auto-expand structures",
        description: "Automatically expand larger data structures",
        default: DefaultValue::Bool(true),
        show_in_sidebar: true,
    },
    Setting {
        id: "auto-group-dumps",
        category: Category::Ui,
        label: "Group dumps",
        description: "Group multiple variables dumped at once",
        default: DefaultValue::Bool(true),
        show_in_sidebar: true,
    },
    Setting {
        id: "long-file-path",
        category: Category::Ui,
        label: "Show full file path",
        description: "Show full path instead of truncated version",
        default: DefaultValue::Bool(false),
        show_in_sidebar: true,
    },
    Setting {
        id: "truncate-large-var-types",
        category: Category::Ui,
        label: "Truncate large types",
        description: "Show truncated version of large variable types.",
        default: DefaultValue::Bool(false),
        show_in_sidebar: true,
    },
    Setting {
        id: "notifications",
        category: Category::Ui,
        label: "Notifications",
        description: "Get a notification when a new payload was received by Quo.",
        default: DefaultValue::Bool(false),
        show_in_sidebar: false,
    },
    Setting {
        id: "server-port",
        category: Category::Server,
        label: "Listening port",
        description: "Port the Quo server listens on",
        default: DefaultValue::Int(7779),
        show_in_sidebar: false,
    },
    Setting {
        id: "analytics",
        category: Category::Privacy,
        label: "Send anonymous analytics",
        description: "Help improve Quo by sending anonymous usage data",
        default: DefaultValue::Bool(false),
        show_in_sidebar: false,
    },
];

pub fn settings_for(category: Category) -> impl Iterator<Item = &'static Setting> {
    SETTINGS.iter().filter(move |s| s.category == category)
}
