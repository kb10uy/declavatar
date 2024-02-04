mod attachment;
mod value;

use crate::decl_v2::Arguments;

use std::rc::Rc;

use ketos::{Module, ModuleBuilder, Scope};

pub const MODULE_NAME_DA3: &str = "da3";

pub fn define_dain_module(scope: Scope, _preprocess: Rc<Arguments>) -> Module {
    attachment::register_attachment_function(&scope);
    value::register_value_function(&scope);

    ModuleBuilder::new(MODULE_NAME_DA3, scope.clone()).finish()
}
