pub struct Config {
    id: String,
    name: String,
    description: String,
    default_value: String,
}

impl Default for Config {
    fn default() -> Self {
        todo!()
    }
}

// ToggleSetting {
//     id: "auto-expand".to_string(),
//     title: "Collapse data".to_string(),
//     description:
//         "Automatically expand larger data structures"
//             .to_string(),
//     position: true,
// },
