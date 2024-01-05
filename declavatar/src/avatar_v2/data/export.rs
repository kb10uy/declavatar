use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type", content = "content")]
pub enum ExportItem {
    Gate { name: String },
    Guard { gate: String, parameter: String },
}
