mod avatar;
mod assets;
mod parameters;

use std::{any::Any, collections::HashMap};

use ketos::{
    Arity, CompileError, Context, Error, ExecError, FromValueRef, Module, ModuleBuilder,
    ModuleLoader, Name, NameStore, Scope, Value,
};

type KetosResult<T> = Result<T, Error>;

#[derive(Debug)]
pub struct DeclavatarModuleLoader;

impl DeclavatarModuleLoader {
    fn define_module(scope: &Scope) -> Module {
        avatar::register_avatar_function(scope);
        parameters::register_parameters_function(scope);
        assets::register_parameters_function(scope);

        ModuleBuilder::new("da", scope.clone()).finish()
    }
}

impl ModuleLoader for DeclavatarModuleLoader {
    fn load_module(&self, name: Name, ctx: Context) -> KetosResult<Module> {
        let load_da = ctx.scope().with_name(name, |n| n == "da");

        if load_da {
            Ok(DeclavatarModuleLoader::define_module(ctx.scope()))
        } else {
            Err(Error::CompileError(CompileError::ModuleError(name)))
        }
    }
}

fn register_function<
    F: Any + for<'a> Fn(&'a NameStore, Name, SeparateArguments<'a>) -> KetosResult<Value>,
>(
    scope: &Scope,
    name: &'static str,
    f: F,
    args_arity: Arity,
    allowed_keywords: &'static [&'static str],
) {
    scope.add_value_with_name(name, |name| {
        Value::new_foreign_fn(name, move |ctx, args| {
            let name_store = ctx.scope().borrow_names();
            let args =
                SeparateArguments::new(&name_store, name, args, args_arity, allowed_keywords)?;
            f(&name_store, name, args)
        })
    });
}

struct SeparateArguments<'a> {
    args: Vec<&'a mut Value>,
    kwargs: HashMap<&'a str, &'a mut Value>,
}

impl<'a> SeparateArguments<'a> {
    fn new(
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

    fn get_arg(&'a self, function_name: Name, index: usize) -> KetosResult<&'a Value> {
        if self.args.len() <= index {
            return Err(Error::ExecError(ExecError::ArityError {
                name: Some(function_name),
                expected: Arity::Min((index + 1) as u32),
                found: self.args.len() as u32,
            }));
        }

        Ok(self.args[index])
    }

    fn exact_arg<T: FromValueRef<'a>>(
        &'a self,
        function_name: Name,
        index: usize,
    ) -> KetosResult<T> {
        let raw_value = self.get_arg(function_name, index)?;
        let value = T::from_value_ref(raw_value)?;
        Ok(value)
    }

    fn args_after(&'a self, function_name: Name, index: usize) -> KetosResult<&'a [&'a mut Value]> {
        if self.args.len() <= index {
            return Err(Error::ExecError(ExecError::ArityError {
                name: Some(function_name),
                expected: Arity::Min((index + 1) as u32),
                found: self.args.len() as u32,
            }));
        }
        Ok(&self.args[index..])
    }

    /*
    fn get_arg_mut(&'a mut self, function_name: Name, index: usize) -> KetosResult<&'a mut Value> {
        if self.args.len() <= index {
            return Err(Error::ExecError(ExecError::ArityError {
                name: Some(function_name),
                expected: Arity::Min((index + 1) as u32),
                found: self.args.len() as u32,
            }));
        }

        Ok(&mut self.args[index])
    }
    */

    fn exact_kwarg<T: FromValueRef<'a>>(&'a self, keyword: &str) -> KetosResult<Option<T>> {
        let Some(value) = self.kwargs.get(keyword) else {
            return Ok(None);
        };

        let value = T::from_value_ref(value)?;
        Ok(Some(value))
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
