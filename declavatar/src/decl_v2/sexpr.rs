mod asset;
mod avatar;
mod controller;
mod driver;
mod layer;
mod menu;
mod parameter;

use crate::decl_v2::{
    data::{avatar::DeclAvatar, StaticTypeName},
    error::{DeclError, DeclSexprError},
};

use std::{any::Any, collections::HashMap, path::PathBuf};

use ketos::{
    Arity, BuiltinModuleLoader, CompileError, Context, Error, ExecError, FileModuleLoader,
    FromValueRef, Interpreter, Module, ModuleBuilder, ModuleLoader, Name, NameStore, Scope, Value,
};

type KetosResult<T> = Result<T, Error>;

#[derive(Debug)]
pub struct DeclavatarModuleLoader;

impl DeclavatarModuleLoader {
    const MODULE_DA: &'static str = "da";
    const MODULE_DA_INTERNAL: &'static str = "da-internal";

    fn get_loader(name: &str) -> Option<fn(Scope) -> Module> {
        match name {
            DeclavatarModuleLoader::MODULE_DA => Some(Self::define_da_module),
            DeclavatarModuleLoader::MODULE_DA_INTERNAL => Some(Self::define_da_internal_module),
            _ => None,
        }
    }

    fn define_da_module(scope: Scope) -> Module {
        avatar::register_avatar_function(&scope);
        parameter::register_parameter_function(&scope);
        asset::register_asset_function(&scope);
        controller::register_controller_function(&scope);
        layer::register_layer_function(&scope);
        driver::register_driver_function(&scope);
        menu::register_menu_function(&scope);

        ModuleBuilder::new(DeclavatarModuleLoader::MODULE_DA, scope.clone()).finish()
    }

    fn define_da_internal_module(scope: Scope) -> Module {
        ModuleBuilder::new(DeclavatarModuleLoader::MODULE_DA_INTERNAL, scope.clone()).finish()
    }
}

impl ModuleLoader for DeclavatarModuleLoader {
    fn load_module(&self, name: Name, ctx: Context) -> KetosResult<Module> {
        let scope = ctx.scope();
        let loader = scope.with_name(name, Self::get_loader);

        match loader {
            Some(l) => Ok(l(scope.clone())),
            None => Err(From::from(CompileError::ModuleError(name))),
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

    fn try_exact_arg<T: FromValueRef<'a>>(&'a self, index: usize) -> KetosResult<Option<T>> {
        let Some(raw_value) = self.args.get(index) else {
            return Ok(None);
        };
        let value = T::from_value_ref(raw_value)?;
        Ok(Some(value))
    }

    fn args_after(&'a self, function_name: Name, index: usize) -> KetosResult<&'a [&'a mut Value]> {
        if self.args.len() < index {
            return Err(Error::ExecError(ExecError::ArityError {
                name: Some(function_name),
                expected: Arity::Min((index + 1) as u32),
                found: self.args.len() as u32,
            }));
        }
        Ok(&self.args[index..])
    }

    fn exact_kwarg<T: FromValueRef<'a>>(&'a self, keyword: &str) -> KetosResult<Option<T>> {
        let Some(value) = self.kwargs.get(keyword) else {
            return Ok(None);
        };

        let value = T::from_value_ref(value)?;
        Ok(Some(value))
    }

    fn exact_kwarg_expect<T: FromValueRef<'a>>(&'a self, keyword: &str) -> KetosResult<T> {
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

trait KetosValueExt {
    fn downcast_foreign_ref<'a, T: FromValueRef<'a> + StaticTypeName>(&'a self) -> KetosResult<T>;
}

impl KetosValueExt for Value {
    fn downcast_foreign_ref<'a, T: FromValueRef<'a> + StaticTypeName>(&'a self) -> KetosResult<T> {
        let expected_type_name = T::TYPE_NAME;
        let found_type_name = self.type_name();
        if found_type_name != expected_type_name {
            return Err(Error::Custom(
                DeclSexprError::UnexpectedTypeValue(
                    found_type_name.to_string(),
                    expected_type_name.to_string(),
                )
                .into(),
            ));
        }

        let value = T::from_value_ref(self)?;
        Ok(value)
    }
}

pub fn load_avatar_sexpr(text: &str, paths: Vec<PathBuf>) -> Result<DeclAvatar, DeclError> {
    let da_loader = DeclavatarModuleLoader;
    let builtin_loader = BuiltinModuleLoader;
    let file_loader = {
        let mut l = FileModuleLoader::with_search_paths(paths);
        l.set_read_bytecode(false);
        l.set_write_bytecode(false);
        l
    };

    let loader = Box::new(da_loader.chain(builtin_loader).chain(file_loader));
    let interpreter = Interpreter::with_loader(loader);

    let result = match interpreter.run_code(text, None) {
        Ok(value) => value,
        Err(kerr) => {
            let error_text = kerr.to_string();
            return Err(DeclError::InternalError(error_text));
        }
    };
    match result.downcast_foreign_ref::<&DeclAvatar>() {
        Ok(avatar) => Ok(avatar.clone()),
        Err(e) => {
            let error_text = e.to_string();
            Err(DeclError::DelclarationNotReturned(Some(error_text)))
        }
    }
}
