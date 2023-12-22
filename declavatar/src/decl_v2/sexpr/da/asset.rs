use crate::decl_v2::{
    data::asset::{DeclAsset, DeclAssets},
    sexpr::{
        argument::{flatten_args_onestep, SeparateArguments},
        error::KetosResult,
        register_function, KetosValueExt,
    },
};

use ketos::{Arity, Name, NameStore, Scope, Value};

pub fn register_asset_function(scope: &Scope) {
    register_function(scope, "assets", declare_assets, Arity::Min(0), &[]);
    register_function(scope, "material", declare_material, Arity::Exact(1), &[]);
    register_function(scope, "animation", declare_animation, Arity::Exact(1), &[]);
}

fn declare_assets(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let mut assets = vec![];
    flatten_args_onestep(args.args_after(function_name, 0)?, |v| {
        assets.push(v.downcast_foreign_ref::<&DeclAsset>().map(|a| a.clone())?);
        Ok(())
    })?;
    Ok(DeclAssets { assets }.into())
}

fn declare_material(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let key: &str = args.exact_arg(function_name, 0)?;
    Ok(DeclAsset::Material(key.to_string()).into())
}

fn declare_animation(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let key: &str = args.exact_arg(function_name, 0)?;
    Ok(DeclAsset::Animation(key.to_string()).into())
}
