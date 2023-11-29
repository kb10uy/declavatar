use crate::decl_sexpr::{
    data::{
        asset::DeclAssets, avatar::DeclAvatar, menu::DeclSubMenu, parameter::DeclParameters,
        StaticTypeName,
    },
    error::DeclError,
    function::{register_function, SeparateArguments},
};

use ketos::{Arity, Error, Name, NameStore, Scope, Value};

use super::KetosValueExt;

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
        menu_blocks: vec![],
    };
    for child_block in args.args_after(function_name, 1)? {
        match child_block.type_name() {
            DeclParameters::TYPE_NAME => {
                let value_ref: &DeclParameters = child_block.downcast_foreign_ref()?;
                avatar.parameters_blocks.push(value_ref.clone());
            }
            DeclAssets::TYPE_NAME => {
                let value_ref: &DeclAssets = child_block.downcast_foreign_ref()?;
                avatar.assets_blocks.push(value_ref.clone());
            }
            DeclSubMenu::TYPE_NAME => {
                let value_ref: &DeclSubMenu = child_block.downcast_foreign_ref()?;
                avatar.menu_blocks.push(value_ref.clone());
            }
            _ => {
                return Err(Error::Custom(
                    DeclError::UnexpectedTypeValue(
                        child_block.type_name().to_string(),
                        "menu element".to_string(),
                    )
                    .into(),
                ))
            }
        }
    }

    Ok(avatar.into())
}
