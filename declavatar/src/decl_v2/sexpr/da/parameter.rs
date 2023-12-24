use crate::decl_v2::{
    data::parameter::{DeclParameter, DeclParameterScope, DeclParameterType, DeclParameters},
    sexpr::{
        argument::SeparateArguments,
        error::{DeclSexprError, KetosResult},
        register_function, KetosValueExt,
    },
};

use ketos::{Arity, Error, Name, NameStore, Scope, Value};

pub fn register_parameter_function(scope: &Scope) {
    const PARAMETER_KEYWORDS: &[&str] = &["save", "default", "scope", "unique"];
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
) -> KetosResult<Value> {
    let mut parameters = vec![];
    for decl_parameter in args.args_after_recursive(function_name, 0)? {
        parameters.push(
            decl_parameter
                .downcast_foreign_ref::<&DeclParameter>()
                .map(|p| p.clone())?,
        );
    }
    Ok(DeclParameters { parameters }.into())
}

fn declare_bool(
    name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let save: Option<bool> = args.exact_kwarg("save")?;
    let default: Option<bool> = args.exact_kwarg("default")?;
    let scope: Option<&Value> = args.exact_kwarg("scope")?;
    let unique: Option<bool> = args.exact_kwarg("unique")?;

    Ok(DeclParameter {
        ty: DeclParameterType::Bool(default),
        name: name.to_string(),
        scope: scope.map(|s| expect_scope(name_store, s)).transpose()?,
        save,
        unique,
    }
    .into())
}

fn declare_int(
    name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let save: Option<bool> = args.exact_kwarg("save")?;
    let default: Option<u8> = args.exact_kwarg("default")?;
    let scope: Option<&Value> = args.exact_kwarg("scope")?;
    let unique: Option<bool> = args.exact_kwarg("unique")?;

    Ok(DeclParameter {
        ty: DeclParameterType::Int(default),
        name: name.to_string(),
        scope: scope.map(|s| expect_scope(name_store, s)).transpose()?,
        save,
        unique,
    }
    .into())
}

fn declare_float(
    name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let save: Option<bool> = args.exact_kwarg("save")?;
    let default: Option<f64> = args.exact_kwarg("default")?;
    let scope: Option<&Value> = args.exact_kwarg("scope")?;
    let unique: Option<bool> = args.exact_kwarg("unique")?;

    Ok(DeclParameter {
        ty: DeclParameterType::Float(default),
        name: name.to_string(),
        scope: scope.map(|s| expect_scope(name_store, s)).transpose()?,
        save,
        unique,
    }
    .into())
}

fn expect_scope(name_store: &NameStore, value: &Value) -> KetosResult<DeclParameterScope> {
    let Value::Name(name) = value else {
        return Err(Error::Custom(DeclSexprError::MustBeScope.into()));
    };

    match name_store.get(*name) {
        "synced" => Ok(DeclParameterScope::Synced),
        "local" => Ok(DeclParameterScope::Local),
        "internal" => Ok(DeclParameterScope::Internal),
        n => Err(Error::Custom(
            DeclSexprError::InvalidScope(n.to_string()).into(),
        )),
    }
}

#[cfg(test)]
mod test {
    use crate::decl_v2::{
        data::parameter::{DeclParameter, DeclParameterScope, DeclParameterType},
        sexpr::test::eval_da_value,
    };

    use pretty_assertions::assert_eq;

    #[test]
    fn reads_int() {
        assert_eq!(
            eval_da_value::<DeclParameter>(r#"(da/int "hoge")"#).expect("should compile"),
            expected_type(DeclParameterType::Int(None))
        );
        assert_eq!(
            eval_da_value::<DeclParameter>(r#"(da/int "hoge" :default 1)"#)
                .expect("should compile"),
            expected_type(DeclParameterType::Int(Some(1)))
        );
    }

    #[test]
    fn reads_bool() {
        assert_eq!(
            eval_da_value::<DeclParameter>(r#"(da/bool "hoge")"#).expect("should compile"),
            expected_type(DeclParameterType::Bool(None))
        );
        assert_eq!(
            eval_da_value::<DeclParameter>(r#"(da/bool "hoge" :default false)"#)
                .expect("should compile"),
            expected_type(DeclParameterType::Bool(Some(false)))
        );
    }

    #[test]
    fn reads_float() {
        assert_eq!(
            eval_da_value::<DeclParameter>(r#"(da/float "hoge")"#).expect("should compile"),
            expected_type(DeclParameterType::Float(None))
        );
        assert_eq!(
            eval_da_value::<DeclParameter>(r#"(da/float "hoge" :default 1.5)"#)
                .expect("should compile"),
            expected_type(DeclParameterType::Float(Some(1.5)))
        );
    }

    #[test]
    fn parses_scope() {
        assert_eq!(
            eval_da_value::<DeclParameter>(r#"(da/int "hoge" :scope 'internal)"#)
                .expect("should compile"),
            expected_scope(DeclParameterScope::Internal)
        );
        assert_eq!(
            eval_da_value::<DeclParameter>(r#"(da/int "hoge" :scope 'local)"#)
                .expect("should compile"),
            expected_scope(DeclParameterScope::Local)
        );
        assert_eq!(
            eval_da_value::<DeclParameter>(r#"(da/int "hoge" :scope 'synced)"#)
                .expect("should compile"),
            expected_scope(DeclParameterScope::Synced)
        );
    }

    #[test]
    fn parses_save() {
        assert_eq!(
            eval_da_value::<DeclParameter>(r#"(da/int "hoge" :save true)"#)
                .expect("should compile"),
            DeclParameter {
                ty: DeclParameterType::Int(None),
                name: "hoge".to_string(),
                scope: None,
                save: Some(true),
                unique: None,
            }
        );
    }

    #[test]
    fn parses_unique() {
        assert_eq!(
            eval_da_value::<DeclParameter>(r#"(da/int "hoge" :unique true)"#)
                .expect("should compile"),
            DeclParameter {
                ty: DeclParameterType::Int(None),
                name: "hoge".to_string(),
                scope: None,
                save: None,
                unique: Some(true),
            }
        );
    }

    fn expected_type(ty: DeclParameterType) -> DeclParameter {
        DeclParameter {
            ty,
            name: "hoge".to_string(),
            scope: None,
            save: None,
            unique: None,
        }
    }

    fn expected_scope(s: DeclParameterScope) -> DeclParameter {
        DeclParameter {
            ty: DeclParameterType::Int(None),
            name: "hoge".to_string(),
            scope: Some(s),
            save: None,
            unique: None,
        }
    }
}
