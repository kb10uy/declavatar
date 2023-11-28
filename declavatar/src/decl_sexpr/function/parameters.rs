use crate::decl_sexpr::{
    data::{DeclParameter, DeclParameterScope, DeclParameterType, DeclParameters},
    error::DeclError,
    function::{register_function, SeparateArguments},
};

use ketos::{Arity, Error, FromValueRef, Name, NameStore, Scope, Value};

pub fn register_parameters_function(scope: &Scope) {
    const PARAMETER_KEYWORDS: &[&str] = &["save", "default", "scope"];
    register_function(scope, "parameters", declare_parameters, Arity::Min(0), &[]);
    register_function(
        scope,
        "bool",
        declare_bool,
        Arity::Exact(1),
        PARAMETER_KEYWORDS,
    );
    register_function(
        scope,
        "int",
        declare_int,
        Arity::Exact(1),
        PARAMETER_KEYWORDS,
    );
    register_function(
        scope,
        "float",
        declare_float,
        Arity::Exact(1),
        PARAMETER_KEYWORDS,
    );
}

fn declare_parameters(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> Result<Value, Error> {
    let mut parameters = vec![];
    for param_value in args.args_after(function_name, 0)? {
        if param_value.type_name() != stringify!(DeclParameter) {
            return Err(Error::Custom(
                DeclError::UnexpectedTypeValue(param_value.type_name().to_string()).into(),
            ));
        }
        let parameter: &DeclParameter = FromValueRef::from_value_ref(param_value)?;
        parameters.push(parameter.clone());
    }
    Ok(DeclParameters { parameters }.into())
}

fn declare_bool(
    name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> Result<Value, Error> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let save: Option<bool> = args.exact_kwarg("save")?;
    let default: Option<bool> = args.exact_kwarg("default")?;
    let scope: Option<&Value> = args.exact_kwarg("scope")?;

    Ok(DeclParameter {
        ty: DeclParameterType::Bool(default),
        scope: scope.map(|s| expect_scope(name_store, s)).transpose()?,
        save,
        name: name.to_string(),
    }
    .into())
}

fn declare_int(
    name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> Result<Value, Error> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let save: Option<bool> = args.exact_kwarg("save")?;
    let default: Option<u8> = args.exact_kwarg("default")?;
    let scope: Option<&Value> = args.exact_kwarg("scope")?;

    Ok(DeclParameter {
        ty: DeclParameterType::Int(default),
        scope: scope.map(|s| expect_scope(name_store, s)).transpose()?,
        save,
        name: name.to_string(),
    }
    .into())
}

fn declare_float(
    name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> Result<Value, Error> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let save: Option<bool> = args.exact_kwarg("save")?;
    let default: Option<f64> = args.exact_kwarg("default")?;
    let scope: Option<&Value> = args.exact_kwarg("scope")?;

    Ok(DeclParameter {
        ty: DeclParameterType::Float(default),
        scope: scope.map(|s| expect_scope(name_store, s)).transpose()?,
        save,
        name: name.to_string(),
    }
    .into())
}

fn expect_scope(name_store: &NameStore, value: &Value) -> Result<DeclParameterScope, Error> {
    let Value::Name(name) = value else {
        return Err(Error::Custom(DeclError::MustBeScope.into()));
    };

    match name_store.get(*name) {
        "synced" => Ok(DeclParameterScope::Synced),
        "local" => Ok(DeclParameterScope::Local),
        "internal" => Ok(DeclParameterScope::Internal),
        n => Err(Error::Custom(DeclError::InvalidScope(n.to_string()).into())),
    }
}
