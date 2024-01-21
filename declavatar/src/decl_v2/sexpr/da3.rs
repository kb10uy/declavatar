mod value;

use crate::decl_v2::PreprocessData;

use std::rc::Rc;

use ketos::{Module, ModuleBuilder, Scope};

pub const MODULE_NAME_DA3: &str = "da3";

pub fn define_dain_module(scope: Scope, _preprocess: Rc<PreprocessData>) -> Module {
    value::register_value_function(&scope);

    ModuleBuilder::new(MODULE_NAME_DA3, scope.clone()).finish()
}
