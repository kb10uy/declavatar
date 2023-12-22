use crate::decl_v2::{
    data::{controller::DeclFxController, layer::DeclControllerLayer},
    sexpr::{
        argument::{flatten_args_onestep, SeparateArguments},
        error::KetosResult,
        register_function, KetosValueExt,
    },
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
    flatten_args_onestep(args.args_after(function_name, 0)?, |l| {
        layers.push(l.downcast_foreign_ref::<&DeclControllerLayer>()?.clone());
        Ok(())
    })?;
    Ok(DeclFxController { layers }.into())
}
