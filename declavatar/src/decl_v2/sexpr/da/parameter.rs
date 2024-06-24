use crate::decl_v2::{
    data::parameter::{
        DeclParameter, DeclParameters, DeclPhysBoneParameter, DeclPrimitiveParameter,
        DeclPrimitiveParameterScope, DeclPrimitiveParameterType, DeclProvidedParameterKind,
    },
    sexpr::{
        argument::SeparateArguments,
        error::{DeclSexprError, KetosResult},
        register_function, KetosValueExt,
    },
};

use ketos::{Arity, Error, Name, NameStore, Scope, Value};

pub fn register_parameter_function(scope: &Scope) {
    const PARAMETER_KEYWORDS: &[&str] = &["save", "default", "scope", "unique"];
    register_function(
        scope,
        "parameters",
        declare_parameters,
        Arity::Min(0),
        Some(&[]),
    );
    register_function(
        scope,
        "bool",
        declare_bool,
        Arity::Exact(1),
        Some(PARAMETER_KEYWORDS),
    );
    register_function(
        scope,
        "int",
        declare_int,
        Arity::Exact(1),
        Some(PARAMETER_KEYWORDS),
    );
    register_function(
        scope,
        "float",
        declare_float,
        Arity::Exact(1),
        Some(PARAMETER_KEYWORDS),
    );
    register_function(
        scope,
        "vrc-paramset",
        declare_vrc_paramset,
        Arity::Min(0),
        Some(&[]),
    );
    register_function(
        scope,
        "pb-paramset",
        declare_pb_paramset,
        Arity::Exact(1),
        Some(&[]),
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
                .cloned()?,
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

    Ok(DeclParameter::Primitive(DeclPrimitiveParameter {
        ty: DeclPrimitiveParameterType::Bool(default),
        name: name.to_string(),
        scope: scope.map(|s| expect_scope(name_store, s)).transpose()?,
        save,
        unique,
    })
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

    Ok(DeclParameter::Primitive(DeclPrimitiveParameter {
        ty: DeclPrimitiveParameterType::Int(default),
        name: name.to_string(),
        scope: scope.map(|s| expect_scope(name_store, s)).transpose()?,
        save,
        unique,
    })
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

    Ok(DeclParameter::Primitive(DeclPrimitiveParameter {
        ty: DeclPrimitiveParameterType::Float(default),
        name: name.to_string(),
        scope: scope.map(|s| expect_scope(name_store, s)).transpose()?,
        save,
        unique,
    })
    .into())
}

fn expect_scope(name_store: &NameStore, value: &Value) -> KetosResult<DeclPrimitiveParameterScope> {
    let Value::Name(name) = value else {
        return Err(Error::Custom(DeclSexprError::MustBeScope.into()));
    };

    match name_store.get(*name) {
        "synced" => Ok(DeclPrimitiveParameterScope::Synced),
        "local" => Ok(DeclPrimitiveParameterScope::Local),
        "internal" => Ok(DeclPrimitiveParameterScope::Internal),
        n => Err(Error::Custom(
            DeclSexprError::InvalidScope(n.to_string()).into(),
        )),
    }
}

fn declare_vrc_paramset(
    name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let mut kinds = vec![];
    for decl_kind in args.args_after_recursive(function_name, 0)? {
        let kind = expect_provided_kind(name_store, decl_kind)?;
        kinds.push(kind);
    }

    Ok(DeclParameter::Provided(kinds).into())
}

fn expect_provided_kind(
    name_store: &NameStore,
    value: &Value,
) -> KetosResult<DeclProvidedParameterKind> {
    let Value::Name(name) = value else {
        return Err(Error::Custom(DeclSexprError::MustBeScope.into()));
    };

    match name_store.get(*name).parse() {
        Ok(kind) => Ok(kind),
        Err(n) => Err(Error::Custom(
            DeclSexprError::InvalidVrchatParameter(n).into(),
        )),
    }
}

fn declare_pb_paramset(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let prefix: &str = args.exact_arg(function_name, 0)?;
    Ok(DeclParameter::PhysBone(DeclPhysBoneParameter {
        prefix: prefix.to_string(),
    })
    .into())
}

#[cfg(test)]
mod test {
    use crate::decl_v2::{
        data::parameter::{
            DeclParameter, DeclParameters, DeclPrimitiveParameter, DeclPrimitiveParameterScope,
            DeclPrimitiveParameterType,
        },
        sexpr::test::eval_da_value,
    };

    use pretty_assertions::assert_eq;

    #[test]
    fn reads_parameters() {
        assert_eq!(
            eval_da_value::<DeclParameters>(r#"(da/parameters)"#)
                .parameters
                .len(),
            0
        );
        assert_eq!(
            eval_da_value::<DeclParameters>(r#"(da/parameters (da/bool "hoge"))"#)
                .parameters
                .len(),
            1
        );
        assert_eq!(
            eval_da_value::<DeclParameters>(
                r#"(da/parameters (list (da/bool "hoge") (da/int "fuga")))"#
            )
            .parameters
            .len(),
            2
        );
    }

    #[test]
    fn reads_int() {
        assert_eq!(
            eval_da_value::<DeclParameter>(r#"(da/int "hoge")"#),
            expected_type(DeclPrimitiveParameterType::Int(None))
        );
        assert_eq!(
            eval_da_value::<DeclParameter>(r#"(da/int "hoge" :default 1)"#),
            expected_type(DeclPrimitiveParameterType::Int(Some(1)))
        );
    }

    #[test]
    fn reads_bool() {
        assert_eq!(
            eval_da_value::<DeclParameter>(r#"(da/bool "hoge")"#),
            expected_type(DeclPrimitiveParameterType::Bool(None))
        );
        assert_eq!(
            eval_da_value::<DeclParameter>(r#"(da/bool "hoge" :default false)"#),
            expected_type(DeclPrimitiveParameterType::Bool(Some(false)))
        );
    }

    #[test]
    fn reads_float() {
        assert_eq!(
            eval_da_value::<DeclParameter>(r#"(da/float "hoge")"#),
            expected_type(DeclPrimitiveParameterType::Float(None))
        );
        assert_eq!(
            eval_da_value::<DeclParameter>(r#"(da/float "hoge" :default 1.5)"#),
            expected_type(DeclPrimitiveParameterType::Float(Some(1.5)))
        );
    }

    #[test]
    fn parses_scope() {
        assert_eq!(
            eval_da_value::<DeclParameter>(r#"(da/int "hoge" :scope 'internal)"#),
            expected_scope(DeclPrimitiveParameterScope::Internal)
        );
        assert_eq!(
            eval_da_value::<DeclParameter>(r#"(da/int "hoge" :scope 'local)"#),
            expected_scope(DeclPrimitiveParameterScope::Local)
        );
        assert_eq!(
            eval_da_value::<DeclParameter>(r#"(da/int "hoge" :scope 'synced)"#),
            expected_scope(DeclPrimitiveParameterScope::Synced)
        );
    }

    #[test]
    fn parses_save() {
        assert_eq!(
            eval_da_value::<DeclParameter>(r#"(da/int "hoge" :save true)"#),
            DeclParameter::Primitive(DeclPrimitiveParameter {
                ty: DeclPrimitiveParameterType::Int(None),
                name: "hoge".to_string(),
                scope: None,
                save: Some(true),
                unique: None,
            })
        );
    }

    #[test]
    fn parses_unique() {
        assert_eq!(
            eval_da_value::<DeclParameter>(r#"(da/int "hoge" :unique true)"#),
            DeclParameter::Primitive(DeclPrimitiveParameter {
                ty: DeclPrimitiveParameterType::Int(None),
                name: "hoge".to_string(),
                scope: None,
                save: None,
                unique: Some(true),
            })
        );
    }

    fn expected_type(ty: DeclPrimitiveParameterType) -> DeclParameter {
        DeclParameter::Primitive(DeclPrimitiveParameter {
            ty,
            name: "hoge".to_string(),
            scope: None,
            save: None,
            unique: None,
        })
    }

    fn expected_scope(s: DeclPrimitiveParameterScope) -> DeclParameter {
        DeclParameter::Primitive(DeclPrimitiveParameter {
            ty: DeclPrimitiveParameterType::Int(None),
            name: "hoge".to_string(),
            scope: Some(s),
            save: None,
            unique: None,
        })
    }
}
