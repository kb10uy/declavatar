pub(super) mod asset;
pub(super) mod avatar;
pub(super) mod controller;
pub(super) mod driver;
pub(super) mod layer_basic;
pub(super) mod layer_raw;
pub(super) mod menu;
pub(super) mod parameter;

use ketos::{Module, ModuleBuilder, Scope};

pub const MODULE_NAME_DA: &str = "da";

pub fn define_da_module(scope: Scope) -> Module {
    avatar::register_avatar_function(&scope);
    parameter::register_parameter_function(&scope);
    asset::register_asset_function(&scope);
    controller::register_controller_function(&scope);
    layer_basic::register_layer_basic_function(&scope);
    layer_raw::register_layer_raw_function(&scope);
    driver::register_driver_function(&scope);
    menu::register_menu_function(&scope);

    ModuleBuilder::new(MODULE_NAME_DA, scope.clone()).finish()
}
