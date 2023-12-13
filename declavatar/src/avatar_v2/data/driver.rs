use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "content")]
pub enum ParameterDrive {
    SetInt(String, u8),
    SetFloat(String, f64),
    SetBool(String, bool),
    AddInt(String, u8),
    AddFloat(String, f64),
    RandomInt(String, (u8, u8)),
    RandomFloat(String, (f64, f64)),
    RandomBool(String, f64),
    Copy(String, String),
    RangedCopy(String, String, (f64, f64), (f64, f64)),
}
