use crate::decl_v2::sexpr::{error::DeclSexprError, KetosResult};

use std::collections::HashMap;

use ketos::{Arity, Error, ExecError, FromValueRef, Name, NameStore, Value};

pub struct SeparateArguments<'a> {
    args: Vec<&'a mut Value>,
    kwargs: HashMap<&'a str, &'a mut Value>,
}

impl<'a> SeparateArguments<'a> {
    pub fn new(
        name_store: &'a NameStore,
        function_name: Name,
        raw_args: &'a mut [Value],
        args_arity: Arity,
        allowed_keywords: &'static [&'static str],
    ) -> KetosResult<SeparateArguments<'a>> {
        let (args, kwargs) = SeparateArguments::separate_args(
            name_store,
            function_name,
            raw_args,
            args_arity,
            allowed_keywords,
        )?;
        Ok(SeparateArguments { args, kwargs })
    }

    pub fn get_arg(&'a self, function_name: Name, index: usize) -> KetosResult<&'a Value> {
        if self.args.len() <= index {
            return Err(Error::ExecError(ExecError::ArityError {
                name: Some(function_name),
                expected: Arity::Min((index + 1) as u32),
                found: self.args.len() as u32,
            }));
        }

        Ok(self.args[index])
    }

    pub fn exact_arg<T: FromValueRef<'a>>(
        &'a self,
        function_name: Name,
        index: usize,
    ) -> KetosResult<T> {
        let raw_value = self.get_arg(function_name, index)?;
        let value = T::from_value_ref(raw_value)?;
        Ok(value)
    }

    pub fn try_exact_arg<T: FromValueRef<'a>>(&'a self, index: usize) -> KetosResult<Option<T>> {
        let Some(raw_value) = self.args.get(index) else {
            return Ok(None);
        };
        let value = T::from_value_ref(raw_value)?;
        Ok(Some(value))
    }

    pub fn args_after(
        &'a self,
        function_name: Name,
        index: usize,
    ) -> KetosResult<&'a [&'a mut Value]> {
        if self.args.len() < index {
            return Err(Error::ExecError(ExecError::ArityError {
                name: Some(function_name),
                expected: Arity::Min((index + 1) as u32),
                found: self.args.len() as u32,
            }));
        }
        Ok(&self.args[index..])
    }

    pub fn exact_kwarg<T: FromValueRef<'a>>(&'a self, keyword: &str) -> KetosResult<Option<T>> {
        let Some(value) = self.kwargs.get(keyword) else {
            return Ok(None);
        };

        let value = T::from_value_ref(value)?;
        Ok(Some(value))
    }

    pub fn exact_kwarg_expect<T: FromValueRef<'a>>(&'a self, keyword: &str) -> KetosResult<T> {
        let Some(value) = self.kwargs.get(keyword) else {
            return Err(Error::Custom(
                DeclSexprError::KeywordExpected(keyword.to_string()).into(),
            ));
        };

        let value = T::from_value_ref(value)?;
        Ok(value)
    }

    fn separate_args(
        name_store: &'a NameStore,
        function_name: Name,
        values: &'a mut [Value],
        args_arity: Arity,
        allowed_keywords: &'static [&'static str],
    ) -> KetosResult<(Vec<&'a mut Value>, HashMap<&'a str, &'a mut Value>)> {
        let mut args = vec![];
        let mut kwargs = HashMap::new();

        let mut values_iter = values.iter_mut();
        while let Some(value) = values_iter.next() {
            match value {
                Value::Keyword(name) => {
                    // continuous keywords are recognized as "passed a keyword for a kwarg"
                    let v = values_iter
                        .next()
                        .ok_or(Error::ExecError(ExecError::OddKeywordParams))?;

                    let real_name = name_store.get(*name);
                    if !allowed_keywords.contains(&real_name) {
                        return Err(Error::ExecError(ExecError::UnrecognizedKeyword(*name)));
                    }

                    kwargs.insert(real_name, v);
                }
                v => {
                    args.push(v);
                }
            }
        }

        if !args_arity.accepts(args.len() as u32) {
            return Err(Error::ExecError(ExecError::ArityError {
                name: Some(function_name),
                expected: args_arity,
                found: args.len() as u32,
            }));
        }

        Ok((args, kwargs))
    }
}