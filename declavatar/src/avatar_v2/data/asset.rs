use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Asset {
    pub asset_type: AssetType,
    pub key: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum AssetType {
    Material,
    Animation,
}

impl AssetType {
    pub const fn type_name(self) -> &'static str {
        match self {
            AssetType::Material => "material",
            AssetType::Animation => "animation",
        }
    }
}
