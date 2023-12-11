use crate::avatar_v2::data::parameter::ParameterType;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type", content = "content")]
pub enum MenuItem {
    SubMenu(MenuGroup),
    Button(MenuBoolean),
    Toggle(MenuBoolean),
    Radial(MenuRadial),
    TwoAxis(MenuTwoAxis),
    FourAxis(MenuFourAxis),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MenuGroup {
    pub name: String,
    pub items: Vec<MenuItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MenuBoolean {
    pub name: String,
    pub parameter: String,
    pub value: ParameterType,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MenuRadial {
    pub name: String,
    pub parameter: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MenuTwoAxis {
    pub name: String,
    pub horizontal_axis: BiAxis,
    pub vertical_axis: BiAxis,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MenuFourAxis {
    pub name: String,
    pub left_axis: UniAxis,
    pub right_axis: UniAxis,
    pub up_axis: UniAxis,
    pub down_axis: UniAxis,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BiAxis {
    pub parameter: String,
    pub label_positive: String,
    pub label_negative: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UniAxis {
    pub parameter: String,
    pub label: String,
}
