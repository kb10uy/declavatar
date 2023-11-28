use crate::decl_sexpr::function::SeparateArguments;

use ketos::{Error, Name, NameStore, Value};

fn declare_avatar(
    name_store: &NameStore,
    function_name: Name,
    mut args: SeparateArguments,
) -> Result<Value, Error> {
    let name: &str = args.exact_arg(function_name, 0)?;
    Ok(name.into())
}
