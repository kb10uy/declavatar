use crate::arbittach::{data::Value, error::TypeError, schema::ValueType};

use std::iter::zip;

pub fn validate_type(value: &Value, value_type: &ValueType) -> Result<(), TypeError> {
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
