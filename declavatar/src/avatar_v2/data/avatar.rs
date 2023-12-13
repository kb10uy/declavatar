use crate::avatar_v2::data::{asset::Asset, layer::Layer, menu::MenuItem, parameter::Parameter};

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Avatar {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub assets: Vec<Asset>,
    pub fx_controller: Vec<Layer>,
    pub menu_items: Vec<MenuItem>,
}
