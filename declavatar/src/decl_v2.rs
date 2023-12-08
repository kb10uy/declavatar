pub mod data;
pub mod error;
mod sexpr;

use crate::decl_v2::sexpr::load_avatar_sexpr;

#[derive(Debug, Clone)]
pub struct Avatar {}

pub fn run_test() {
    match load_avatar_sexpr(include_str!("../../examples/sexpr-all.decl.lisp")) {
        Ok(v) => {
            println!("{v:?}");
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }
}
