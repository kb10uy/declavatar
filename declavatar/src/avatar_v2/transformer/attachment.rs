use crate::{
    avatar_v2::{
        data::attachment::{
            schema::{Attachment as AttachmentSchema, ValueType},
            Attachment, AttachmentGroup, Property, Value,
        },
        log::{ArbittachError, Log},
        transformer::{failure, success, Compiled},
    },
    decl_v2::data::arbittach::{
        DeclAttachment, DeclAttachmentGroup, DeclAttachmentProperty, DeclAttachments,
    },
    log::Logger,
};

use std::{collections::HashMap, iter::zip};

pub fn compile_attachment_blocks(
    logger: &Logger<Log>,
    schemas: &HashMap<String, AttachmentSchema>,
    attachment_blocks: Vec<DeclAttachments>,
) -> Compiled<Vec<AttachmentGroup>> {
    let mut attachment_groups = vec![];
    for (index, decl_attachments) in attachment_blocks.into_iter().enumerate() {
        let logger = logger.with_context(format!("attachments block {index}"));
        for decl_group in decl_attachments.targets {
            let Some(group) = compile_group(&logger, schemas, decl_group) else {
                continue;
            };
            attachment_groups.push(group);
        }
    }

    success(attachment_groups)
}

fn compile_group(
    logger: &Logger<Log>,
    schemas: &HashMap<String, AttachmentSchema>,
    decl_group: DeclAttachmentGroup,
) -> Compiled<AttachmentGroup> {
    let target = decl_group.target;
    let mut attachments = vec![];

    let logger = logger.with_context(format!("target {target}"));
    for decl_attachment in decl_group.attachments {
        let Some(attachment) = compile_attachment(&logger, schemas, decl_attachment) else {
            continue;
        };
        attachments.push(attachment);
    }

    success(AttachmentGroup {
        target,
        attachments,
    })
}

fn compile_attachment(
    logger: &Logger<Log>,
    schemas: &HashMap<String, AttachmentSchema>,
    decl_attachment: DeclAttachment,
) -> Compiled<Attachment> {
    let name = decl_attachment.name;
    let mut properties = vec![];

    let logger = logger.with_context(format!("attachment {name}"));
    for decl_property in decl_attachment.properties {
        let Some(property) = compile_property(&logger, schemas, decl_property) else {
            continue;
        };
        properties.push(property);
    }

    success(Attachment { name, properties })
}

fn compile_property(
    logger: &Logger<Log>,
    schemas: &HashMap<String, AttachmentSchema>,
    decl_property: DeclAttachmentProperty,
) -> Compiled<Property> {
    todo!()
}

fn validate_type(logger: &Logger<Log>, value: &Value, value_type: &ValueType) -> Compiled<()> {
    match (value_type, value) {
        (ValueType::Any, v) => (),
        (ValueType::OneOf(types), v) => {
            let accepted = types
                .iter()
                .map(|t| validate_type(logger, v, t))
                .any(|v| v.is_some());
            if !accepted {
                logger.log(Log::InvalidArbittach(ArbittachError::TypeMismatch {
                    found: v.type_name().to_string(),
                    expected: "specific types".to_string(),
                }));
                return failure();
            }
        }
        (ValueType::List(t), Value::List(values)) => {
            let accepted = values
                .iter()
                .map(|v| validate_type(logger, v, t))
                .any(|v| v.is_some());
            if !accepted {
                logger.log(Log::InvalidArbittach(ArbittachError::TypeMismatch {
                    found: "inacceptable types".to_string(),
                    expected: t.name().to_string(),
                }));
                return failure();
            }
        }
        (ValueType::Tuple(types), Value::Tuple(values)) => {
            if types.len() != values.len() {
                logger.log(Log::InvalidArbittach(ArbittachError::LengthMismatch {
                    found: values.len(),
                    expected: types.len(),
                }));
                return failure();
            }
            zip(types, values).try_fold((), |_, (t, v)| validate_type(logger, v, t))?;
        }
        (ValueType::Map(_kt, _vt), _) => {
            logger.log(Log::InvalidArbittach(ArbittachError::UnsupportedType));
            return failure();
        }
        (ValueType::Null, Value::Null) => (),
        (ValueType::Boolean, Value::Boolean(_)) => (),
        (ValueType::Integer, Value::Integer(_)) => (),
        (ValueType::Float, Value::Float(_)) => (),
        (ValueType::String, Value::String(_)) => (),
        (ValueType::Vector(length), Value::Vector(values)) => {
            if *length == values.len() {
            } else {
                logger.log(Log::InvalidArbittach(ArbittachError::LengthMismatch {
                    found: values.len(),
                    expected: *length,
                }));
                return failure();
            }
        }
        (ValueType::GameObject, Value::GameObject(_)) => (),
        (ValueType::Material, Value::Material(_)) => (),
        (ValueType::AnimationClip, Value::AnimationClip(_)) => (),
        (t, v) => {
            logger.log(Log::InvalidArbittach(ArbittachError::TypeMismatch {
                found: v.type_name().to_string(),
                expected: t.name().to_string(),
            }));
            return failure();
        }
    }
    success(())
}
