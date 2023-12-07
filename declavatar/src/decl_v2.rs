mod data;
mod error;
mod sexpr;

use ketos::{BuiltinModuleLoader, Interpreter, ModuleLoader};

use crate::decl_v2::sexpr::DeclavatarModuleLoader;

#[derive(Debug, Clone)]
pub struct Avatar {}

pub fn run_test() {
    let interp =
        Interpreter::with_loader(Box::new(DeclavatarModuleLoader.chain(BuiltinModuleLoader)));

    let result = interp.run_code(include_str!("../../examples/sexpr-all.decl.lisp"), None);
    match result {
        Ok(v) => {
            println!("{v:?}");
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }
}
