use std::any::Any;

use ketos::{Arity, Context, Error, ExecError, FromValue, FromValueRef, Name, Scope, Value};

#[derive(Debug, Clone)]
pub struct Avatar {}

fn avatar(function_name: Name, values: &mut [Value]) -> Result<Value, Error> {
    let name: &str = exact_arg(function_name, values, 0)?;
    Ok(name.into())
}

fn register_function<F: Any + Fn(Name, &mut [Value]) -> Result<Value, Error>>(
    scope: Scope,
    name: &'static str,
    f: F,
) {
    scope.add_value_with_name(name, |name| {
        Value::new_foreign_fn(name, move |_, args| f(name, args))
    });
}

fn exact_arg<'a, T: FromValueRef<'a>>(
    function_name: Name,
    values: &'a [Value],
    index: usize,
) -> Result<T, Error> {
    if values.len() <= index {
        return Err(Error::ExecError(ExecError::ArityError {
            name: Some(function_name),
            expected: Arity::Min((index + 1) as u32),
            found: values.len() as u32,
        }));
    }

    let value = T::from_value_ref(&values[index])?;
    Ok(value)
}

fn args_after(function_name: Name, values: &[Value], take_from: usize) -> Result<&[Value], Error> {
    if values.len() <= take_from {
        return Err(Error::ExecError(ExecError::ArityError {
            name: Some(function_name),
            expected: Arity::Min((take_from + 1) as u32),
            found: values.len() as u32,
        }));
    }

    Ok(&values[take_from..])
}
