use std::iter::zip;

use crate::{
    arbittach::schema::ValueType,
    avatar_v2::{
        data::attachment::{AttachmentGroup, Value},
        log::Log,
        transformer::{failure, success, Compiled},
    },
    decl_v2::data::arbittach::{DeclAttachmentGroup, DeclAttachments},
    log::Logger,
};

pub fn compile_attachment_blocks(
    logger: &Logger<Log>,
    attachment_blocks: Vec<DeclAttachments>,
) -> Compiled<Vec<AttachmentGroup>> {
    let mut attachment_groups = vec![];
    for (index, decl_attachments) in attachment_blocks.into_iter().enumerate() {
        let logger = logger.with_context(format!("assets block {index}"));
        for decl_target in decl_attachments.targets {
            let Some(group) = compile_group(&logger, decl_target) else {
                continue;
            };
            attachment_groups.push(group);
        }
    }

    success(attachment_groups)
}

fn compile_group(
    logger: &Logger<Log>,
    decl_group: Vec<DeclAttachmentGroup>,
) -> Compiled<AttachmentGroup> {
}

fn validate_type(value: &Value, value_type: &ValueType) -> Result<(), TypeError> {
    match (value_type, value) {
        (ValueType::Any, _) => (),
        (ValueType::OneOf(types), v) => types.iter().try_fold((), |_, t| validate_type(v, t))?,
        (ValueType::List(t), Value::List(values)) => {
            values.iter().try_fold((), |_, v| validate_type(v, t))?;
        }
        (ValueType::Tuple(types), Value::Tuple(values)) => {
            if types.len() != values.len() {
                return Err(TypeError::LengthMismatch {
                    found: values.len(),
                    expected: types.len(),
                });
            }
            zip(types, values).try_fold((), |_, (t, v)| validate_type(v, t))?;
        }
        (ValueType::Map(_kt, _vt), _) => {
            return Err(TypeError::UnsupportedType);
        }
        (ValueType::Null, Value::Null) => (),
        (ValueType::Boolean, Value::Boolean(_)) => (),
        (ValueType::Integer, Value::Integer(_)) => (),
        (ValueType::Float, Value::Float(_)) => (),
        (ValueType::String, Value::String(_)) => (),
        (ValueType::Vector(length), Value::Vector(values)) => {
            if *length != values.len() {
                return Err(TypeError::LengthMismatch {
                    found: values.len(),
                    expected: *length,
                });
            }
        }
        (ValueType::GameObject, Value::GameObject(_)) => (),
        (ValueType::Material, Value::Material(_)) => (),
        (ValueType::AnimationClip, Value::AnimationClip(_)) => (),
        (t, v) => {
            return Err(TypeError::TypeMismatch {
                found: v.type_name().to_string(),
                expected: t.name().to_string(),
            });
        }
    }
    Ok(())
}
