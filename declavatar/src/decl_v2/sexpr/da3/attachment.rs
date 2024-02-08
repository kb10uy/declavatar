use crate::decl_v2::{
    data::{
        arbittach::{DeclAttachment, DeclAttachmentProperty, DeclAttachmentValue, DeclAttachments},
        StaticTypeName,
    },
    sexpr::{argument::SeparateArguments, error::KetosResult, register_function, KetosValueExt},
};

use ketos::{Arity, Error, Name, NameStore, Scope, Value};

pub fn register_attachment_function(scope: &Scope) {
    register_function(
        scope,
        "attachments",
        declare_attachments,
        Arity::Min(0),
        Some(&[]),
    );
    register_function(
        scope,
        "attachment",
        define_attachment,
        Arity::Min(1),
        Some(&[]),
    );
    register_function(scope, "property", define_property, Arity::Min(1), None);
}

fn declare_attachments(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let mut attachments = vec![];
    for decl_attachment in args.args_after_recursive(function_name, 0)? {
        attachments.push(
            decl_attachment
                .downcast_foreign_ref::<&DeclAttachment>()
                .cloned()?,
        );
    }
    Ok(DeclAttachments { attachments }.into())
}

fn define_attachment(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;

    let mut properties = vec![];
    for decl_property in args.args_after_recursive(function_name, 1)? {
        properties.push(
            decl_property
                .downcast_foreign_ref::<&DeclAttachmentProperty>()?
                .clone(),
        );
    }
    Ok(DeclAttachment {
        name: name.to_string(),
        properties,
    }
    .into())
}

fn define_property(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;

    let mut parameters = vec![];
    for decl_paramerter in args.args_after_recursive(function_name, 1)? {
        parameters.push(parse_attachment_value(decl_paramerter)?);
    }
    Ok(DeclAttachmentProperty {
        name: name.to_string(),
        parameters,
    }
    .into())
}

fn parse_attachment_value(value: &Value) -> KetosResult<DeclAttachmentValue> {
    let attachment_value = match value {
        Value::Unit => DeclAttachmentValue::Null,
        Value::Bool(v) => DeclAttachmentValue::Boolean(*v),
        Value::Integer(v) => DeclAttachmentValue::Integer(v.to_i64().unwrap_or(0)),
        Value::Float(v) => DeclAttachmentValue::Float(*v),
        Value::String(v) => DeclAttachmentValue::String(v.to_string()),
        Value::List(values) => values
            .iter()
            .map(parse_attachment_value)
            .collect::<Result<Vec<_>, _>>()
            .map(DeclAttachmentValue::UntypedList)?,
        Value::Foreign(_) if value.type_name() == DeclAttachmentValue::TYPE_NAME => value
            .downcast_foreign_ref::<&DeclAttachmentValue>()?
            .clone(),
        v => {
            return Err(Error::ExecError(ketos::ExecError::TypeError {
                expected: "unit, bool, integer, float, string, list, or specific object",
                found: "incompatible type",
                value: Some(v.clone()),
            }))
        }
    };
    Ok(attachment_value)
}
