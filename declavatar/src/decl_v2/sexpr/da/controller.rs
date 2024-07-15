use crate::decl_v2::{
    data::{controller::DeclFxController, layer::DeclControllerLayer},
    sexpr::{argument::SeparateArguments, error::KetosResult, register_function, KetosValueExt},
};

use ketos::{Arity, Name, NameStore, Scope, Value};

pub fn register_controller_function(scope: &Scope) {
    register_function(scope, "fx-controller", declare_fx_controller, Arity::Min(0), Some(&[]));
}

fn declare_fx_controller(_name_store: &NameStore, function_name: Name, args: SeparateArguments) -> KetosResult<Value> {
    let mut layers = vec![];
    for decl_layer in args.args_after_recursive(function_name, 0)? {
        layers.push(decl_layer.downcast_foreign_ref::<&DeclControllerLayer>()?.clone());
    }
    Ok(DeclFxController { layers }.into())
}
