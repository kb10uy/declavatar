use crate::decl_sexpr::{
    data::{DeclAsset, DeclAssets},
    error::DeclError,
    function::{register_function, SeparateArguments},
};

use ketos::{Arity, Error, FromValueRef, Name, NameStore, Scope, Value};

pub fn register_parameters_function(scope: &Scope) {
    register_function(scope, "assets", declare_assets, Arity::Min(0), &[]);
    register_function(scope, "material", declare_material, Arity::Exact(1), &[]);
    register_function(scope, "animation", declare_animation, Arity::Exact(1), &[]);
}

fn declare_assets(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> Result<Value, Error> {
    let mut assets = vec![];
    for asset_value in args.args_after(function_name, 0)? {
        if asset_value.type_name() != stringify!(DeclAsset) {
            return Err(Error::Custom(
                DeclError::UnexpectedTypeValue(asset_value.type_name().to_string()).into(),
            ));
        }
        let parameter: &DeclAsset = FromValueRef::from_value_ref(asset_value)?;
        assets.push(parameter.clone());
    }
    Ok(DeclAssets { assets }.into())
}

fn declare_material(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> Result<Value, Error> {
    let key: &str = args.exact_arg(function_name, 0)?;
    Ok(DeclAsset::Material(key.to_string()).into())
}

fn declare_animation(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> Result<Value, Error> {
    let key: &str = args.exact_arg(function_name, 0)?;
    Ok(DeclAsset::Animation(key.to_string()).into())
}
