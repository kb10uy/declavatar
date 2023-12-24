mod argument;
mod da;
mod dain;
mod error;

use crate::decl_v2::{
    data::{avatar::DeclAvatar, StaticTypeName},
    error::DeclError,
    sexpr::{
        argument::SeparateArguments,
        error::{DeclSexprError, KetosResult},
    },
};

use std::{any::Any, path::PathBuf};

use ketos::{
    Arity, BuiltinModuleLoader, CompileError, Context, Error, FileModuleLoader, FromValueRef,
    Interpreter, Module, ModuleLoader, Name, NameStore, Scope, Value,
};

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

#[derive(Debug)]
pub struct DeclavatarModuleLoader;

impl DeclavatarModuleLoader {
    fn get_loader(name: &str) -> Option<fn(Scope) -> Module> {
        match name {
            da::MODULE_NAME_DA => Some(da::define_da_module),
            dain::MODULE_NAME_DAIN => Some(dain::define_dain_module),
            _ => None,
        }
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

fn register_function_with_context<
    F: Any + for<'a> Fn(&'a Context, &'a NameStore, Name, SeparateArguments<'a>) -> KetosResult<Value>,
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
            f(ctx, &name_store, name, args)
        })
    });
}

#[cfg(test)]
mod test {
    use super::DeclavatarModuleLoader;
    use crate::decl_v2::data::StaticTypeName;

    use ketos::{BuiltinModuleLoader, FromValue, Interpreter, ModuleLoader};

    pub fn eval_da_value<T: StaticTypeName + FromValue>(source: &str) -> T {
        let da_loader = DeclavatarModuleLoader;
        let builtin_loader = BuiltinModuleLoader;

        let loader = Box::new(da_loader.chain(builtin_loader));
        let interpreter = Interpreter::with_loader(loader);
        interpreter
            .run_code("(use da :self)", None)
            .expect("failed to setup interpreter");

        let value = interpreter
            .run_code(source, None)
            .expect("given source should compile");
        T::from_value(value).expect("wrong type returned")
    }
}
