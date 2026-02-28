use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct ConnectionEstablishedEvent {
    pub host: String,
    pub port: u16,
    pub success: bool,
}
