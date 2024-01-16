use std::rc::Rc;

use crate::decl_v2::{
    sexpr::{argument::SeparateArguments, error::KetosResult, register_function},
    PreprocessData,
};

use ketos::{Arity, Name, Scope, Value};

pub fn register_preprocess_function(scope: &Scope, preprocess: Rc<PreprocessData>) {
    let spp = preprocess.clone();
    register_function(
        scope,
        "symbol",
        move |_, f, a| symbol(f, a, &spp),
        Arity::Exact(1),
        &[],
    );

    let hlpp = preprocess.clone();
    register_function(
        scope,
        "can-localize",
        move |_, f, a| can_localize(f, a, &hlpp),
        Arity::Exact(1),
        &[],
    );

    let hspp = preprocess.clone();
    register_function(
        scope,
        "localize",
        move |_, f, a| localize(f, a, &hspp),
        Arity::Exact(1),
        &[],
    );
}

pub fn symbol(
    function_name: Name,
    args: SeparateArguments,
    preprocess: &PreprocessData,
) -> KetosResult<Value> {
    let symbol_name: &str = args.exact_arg(function_name, 0)?;
    let has_symbol = preprocess.symbols.contains(symbol_name);
    Ok(has_symbol.into())
}

pub fn can_localize(
    function_name: Name,
    args: SeparateArguments,
    preprocess: &PreprocessData,
) -> KetosResult<Value> {
    let localization_key: &str = args.exact_arg(function_name, 0)?;
    let has_localization = preprocess.localizations.contains_key(localization_key);
    Ok(has_localization.into())
}

pub fn localize(
    function_name: Name,
    args: SeparateArguments,
    preprocess: &PreprocessData,
) -> KetosResult<Value> {
    let localization_key: &str = args.exact_arg(function_name, 0)?;
    match preprocess.localizations.get(localization_key) {
        Some(v) => Ok(v.as_str().into()),
        None => Ok(Value::Unit),
    }
}
