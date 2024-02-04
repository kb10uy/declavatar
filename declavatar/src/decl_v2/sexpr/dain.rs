mod option;

use crate::decl_v2::Arguments;

use std::rc::Rc;

use ketos::{Module, ModuleBuilder, Scope};

pub const MODULE_NAME_DAIN: &str = "dain";

pub fn define_dain_module(scope: Scope, _preprocess: Rc<Arguments>) -> Module {
    option::register_option_function(&scope);

    ModuleBuilder::new(MODULE_NAME_DAIN, scope.clone()).finish()
}
