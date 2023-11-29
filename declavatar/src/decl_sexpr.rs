mod data;
mod error;
mod function;

use ketos::{BuiltinModuleLoader, Interpreter, ModuleLoader};

use crate::decl_sexpr::function::DeclavatarModuleLoader;

#[derive(Debug, Clone)]
pub struct Avatar {}

pub fn run_test() {
    let interp =
        Interpreter::with_loader(Box::new(DeclavatarModuleLoader.chain(BuiltinModuleLoader)));

    let value = interp
        .run_code(include_str!("../../examples/sexpr-all.decl.lisp"), None)
        .expect("should");
    println!("{value:?}")
}
