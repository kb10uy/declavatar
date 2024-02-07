use crate::{
    avatar_v2::{
        data::attachment::{
            schema::{Attachment as AttachmentSchema, Property as PropertySchema, ValueType},
            Attachment, AttachmentGroup, Property, Value,
        },
        log::{ArbittachError, Log},
        transformer::{failure, success, Compiled},
    },
    decl_v2::data::arbittach::{
        DeclAttachment, DeclAttachmentGroup, DeclAttachmentProperty, DeclAttachmentValue,
        DeclAttachments,
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
    let Some(schema) = schemas.get(&name) else {
        logger.log(Log::Arbittach(ArbittachError::UnknownAttachment));
        return failure();
    };
    let mut unset_properties: HashMap<&str, &PropertySchema> = schema
        .properties
        .iter()
        .map(|p| (p.name.as_str(), p))
        .collect();
    for decl_property in decl_attachment.properties {
        let Some((_, property_schema)) = unset_properties.remove_entry(decl_property.name.as_str())
        else {
            logger.log(Log::Arbittach(ArbittachError::UnknownProperty(
                decl_property.name.to_string(),
            )));
            continue;
        };

        let Some(property) = compile_property(&logger, property_schema, decl_property) else {
            continue;
        };
        properties.push(property);
    }

    if unset_properties.values().any(|p| p.required) {
        let unmet_properties = unset_properties
            .values()
            .filter(|&p| p.required)
            .map(|p| &p.name)
            .cloned()
            .collect();
        logger.log(Log::Arbittach(ArbittachError::Insufficient {
            unmet_properties,
        }));
        return failure();
    }

    success(Attachment { name, properties })
}

fn compile_property(
    logger: &Logger<Log>,
    property_schema: &PropertySchema,
    decl_property: DeclAttachmentProperty,
) -> Compiled<Property> {
    let mut parameters = vec![];
    let keywords = HashMap::new();

    let logger = logger.with_context(format!("property {}", decl_property.name));
    if property_schema.parameters.len() != decl_property.parameters.len() {
        logger.log(Log::Arbittach(ArbittachError::LengthMismatch {
            expected: property_schema.parameters.len(),
            found: decl_property.parameters.len(),
        }));
    }

    for (dprop, sprop) in zip(decl_property.parameters, &property_schema.parameters) {
        let Some(value) = compile_value(&logger, &sprop.value_type, dprop) else {
            continue;
        };
        parameters.push(value);
    }

    // TODO: parse keywords

    success(Property {
        name: decl_property.name,
        parameters,
        keywords,
    })
}

fn compile_value(
    logger: &Logger<Log>,
    expected_type: &ValueType,
    decl_value: DeclAttachmentValue,
) -> Compiled<Value> {
    let value = match decl_value {
        DeclAttachmentValue::Null if matches!(expected_type, ValueType::Null | ValueType::Any) => {
            Value::Null
        }
        DeclAttachmentValue::Boolean(v)
            if matches!(expected_type, ValueType::Boolean | ValueType::Any) =>
        {
            Value::Boolean(v)
        }
        DeclAttachmentValue::Integer(v)
            if matches!(expected_type, ValueType::Integer | ValueType::Any) =>
        {
            Value::Integer(v)
        }
        DeclAttachmentValue::Float(v)
            if matches!(expected_type, ValueType::Float | ValueType::Any) =>
        {
            Value::Float(v)
        }
        DeclAttachmentValue::String(v)
            if matches!(expected_type, ValueType::String | ValueType::Any) =>
        {
            Value::String(v)
        }
        DeclAttachmentValue::GameObject(v)
            if matches!(expected_type, ValueType::GameObject | ValueType::Any) =>
        {
            Value::GameObject(v)
        }
        DeclAttachmentValue::Material(v)
            if matches!(expected_type, ValueType::Material | ValueType::Any) =>
        {
            Value::Material(v)
        }
        DeclAttachmentValue::AnimationClip(v)
            if matches!(expected_type, ValueType::AnimationClip | ValueType::Any) =>
        {
            Value::AnimationClip(v)
        }

        DeclAttachmentValue::Vector(values) => match expected_type {
            ValueType::Vector(length) if *length == values.len() => Value::Vector(values),
            ValueType::Any => Value::Vector(values),
            ValueType::Vector(expected_length) => {
                logger.log(Log::Arbittach(ArbittachError::LengthMismatch {
                    found: values.len(),
                    expected: *expected_length,
                }));
                return failure();
            }
            _ => {
                logger.log(Log::Arbittach(ArbittachError::TypeMismatch {
                    found: "vector".to_string(),
                    expected: expected_type.name().to_string(),
                }));
                return failure();
            }
        },

        DeclAttachmentValue::UntypedList(untyped_list) => match expected_type {
            ValueType::Any => {
                let any_values = untyped_list
                    .into_iter()
                    .flat_map(|uv| compile_value(logger, &ValueType::Any, uv))
                    .collect();
                Value::List(any_values)
            }
            ValueType::List(item_type) => {
                let Some(typed_values) = untyped_list
                    .into_iter()
                    .map(|uv| compile_value(logger, item_type, uv))
                    .collect::<Option<Vec<_>>>()
                else {
                    // type error has been already logged at this point
                    return failure();
                };
                Value::List(typed_values)
            }
            ValueType::Tuple(item_types) => {
                if untyped_list.len() != item_types.len() {
                    logger.log(Log::Arbittach(ArbittachError::LengthMismatch {
                        found: untyped_list.len(),
                        expected: item_types.len(),
                    }));
                    return failure();
                }
                let Some(typed_values) = zip(untyped_list, item_types)
                    .map(|(uv, t)| compile_value(logger, t, uv))
                    .collect::<Option<Vec<_>>>()
                else {
                    // type error has been already logged at this point
                    return failure();
                };
                Value::Tuple(typed_values)
            }
            _ => {
                logger.log(Log::Arbittach(ArbittachError::TypeMismatch {
                    found: "list".to_string(),
                    expected: expected_type.name().to_string(),
                }));
                return failure();
            }
        },

        _ => {
            logger.log(Log::Arbittach(ArbittachError::TypeMismatch {
                found: "unexpected type value".to_string(),
                expected: expected_type.name().to_string(),
            }));
            return failure();
        }
    };
    success(value)
}
