use serde::Serialize;

// TODO: parameter name should be combined, but separated for compatibility
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

impl ParameterDrive {
    pub fn target_parameter(&self) -> &str {
        match self {
            ParameterDrive::SetInt(p, _) => p,
            ParameterDrive::SetFloat(p, _) => p,
            ParameterDrive::SetBool(p, _) => p,
            ParameterDrive::AddInt(p, _) => p,
            ParameterDrive::AddFloat(p, _) => p,
            ParameterDrive::RandomInt(p, _) => p,
            ParameterDrive::RandomFloat(p, _) => p,
            ParameterDrive::RandomBool(p, _) => p,
            ParameterDrive::Copy(_, p) => p,
            ParameterDrive::RangedCopy(_, p, _, _) => p,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TrackingControl {
    pub animation_desired: bool,
    pub targets: Vec<TrackingTarget>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum TrackingTarget {
    Head,
    Hip,
    Eyes,
    Mouth,
    HandLeft,
    HandRight,
    FootLeft,
    FoorRight,
    FingersLeft,
    FingersRight,
}
