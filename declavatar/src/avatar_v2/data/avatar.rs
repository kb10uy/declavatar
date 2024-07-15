use crate::avatar_v2::data::{
    asset::Asset, attachment::Attachment, export::ExportItem, layer::Layer, menu::MenuItem,
    parameter::DeclaredParameter,
};

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Avatar {
    pub name: String,
    pub exports: Vec<ExportItem>,
    pub attachments: Vec<Attachment>,
    pub parameters: Vec<DeclaredParameter>,
    pub assets: Vec<Asset>,
    pub fx_controller: Vec<Layer>,
    pub menu_items: Vec<MenuItem>,
}
