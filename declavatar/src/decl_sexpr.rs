mod data;
mod error;
mod function;

use ketos::Interpreter;

#[derive(Debug, Clone)]
pub struct Avatar {}

pub fn run_test() {
    let interp = Interpreter::new();
    let scope = interp.scope();
    function::register_decl_functions(scope);

    let value = interp
        .run_code(r#"
            (parameters
                (declare-bool "hoge")
                (declare-int "fuga" :save true :scope 'internal)
                (declare-float "piyo" :default 0.5)
            )
        "#, None)
        .expect("should");
    println!("{value:?}");
}
