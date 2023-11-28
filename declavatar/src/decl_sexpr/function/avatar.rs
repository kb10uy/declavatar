use crate::decl_sexpr::{
    data::{DeclAssets, DeclAvatar, DeclParameters},
    error::DeclError,
    function::{register_function, SeparateArguments},
};

use ketos::{Arity, Error, FromValueRef, Name, NameStore, Scope, Value};

pub fn register_avatar_function(scope: &Scope) {
    register_function(scope, "avatar", declare_avatar, Arity::Min(1), &[]);
}

fn declare_avatar(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> Result<Value, Error> {
    let name: &str = args.exact_arg(function_name, 0)?;

    let mut avatar = DeclAvatar {
        name: name.to_string(),
        parameters_blocks: vec![],
        assets_blocks: vec![],
    };
    for child_block in args.args_after(function_name, 1)? {
        match child_block.type_name() {
            stringify!(DeclParameters) => {
                let value_ref: &DeclParameters = FromValueRef::from_value_ref(child_block)?;
                avatar.parameters_blocks.push(value_ref.clone());
            }
            stringify!(DeclAssets) => {
                let value_ref: &DeclAssets = FromValueRef::from_value_ref(child_block)?;
                avatar.assets_blocks.push(value_ref.clone());
            }
            _ => {
                return Err(Error::Custom(
                    DeclError::UnexpectedTypeValue(child_block.type_name().to_string()).into(),
                ))
            }
        }
    }

    Ok(avatar.into())
}
