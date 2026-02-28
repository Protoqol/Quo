use serde::{Deserialize, Serialize};

// @TODO support for extra custom fields (eg CLI env/Laravel Jobs etc.)

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct QuoPayloadVariable {
    pub var_type: String,  // Type of variable
    pub name: String,      // Name of variable
    pub value: String,     // Value of variable
    pub mutable: bool,     // Is variable mutable?
    pub is_constant: bool, // Is variable a constant?
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct IncomingQuoPayloadMeta {
    pub id: u32,               // ID
    pub uid: String,           // Reproducible UID
    pub origin: String,        // Project name
    pub sender_origin: String, // Filename:line
    pub time_epoch_ms: i64,    // Milliseconds since epoch
    pub variable: QuoPayloadVariable,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum QuoPayloadLanguage {
    Rust,
    Php,
    Python,
    Javascript,
    Typescript,
    Ruby,
    Go,
    #[default]
    Unknown
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct IncomingQuoPayload {
    pub meta: IncomingQuoPayloadMeta,
    pub language: QuoPayloadLanguage,
}
