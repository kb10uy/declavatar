use crate::decl_v2::{
    data::{controller::DeclFxController, layer::DeclControllerLayer},
    sexpr::{register_function, KetosResult, KetosValueExt, SeparateArguments},
};

use ketos::{Arity, Name, NameStore, Scope, Value};

pub fn register_controller_function(scope: &Scope) {
    register_function(
        scope,
        "fx-controller",
        declare_fx_controller,
        Arity::Min(0),
        &[],
    );
}

fn declare_fx_controller(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let mut layers = vec![];
    for layer_value in args.args_after(function_name, 0)? {
        let layer: &DeclControllerLayer = layer_value.downcast_foreign_ref()?;
        layers.push(layer.clone());
    }
    Ok(DeclFxController { layers }.into())
}
