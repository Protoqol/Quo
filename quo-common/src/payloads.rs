use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct QuoPayloadVariable {
    pub var_type: String,
    
    pub name: String,

    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IncomingQuoPayloadMeta {
    pub id: u32,

    pub uid: String,

    pub origin: String,

    pub sender_origin: String,

    pub time: String,

    pub called_variable: String,

    pub variable: QuoPayloadVariable,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct IncomingQuoPayload {
    pub meta: IncomingQuoPayloadMeta,

    pub payload: String,
}

impl IncomingQuoPayload {
    pub fn get_raw_payload(&self) -> String {
        String::from_utf8_lossy(&STANDARD.decode(&self.payload).unwrap()).to_string()
    }
}
