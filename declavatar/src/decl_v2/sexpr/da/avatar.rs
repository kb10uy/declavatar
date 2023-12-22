use crate::decl_v2::{
    data::{
        asset::DeclAssets, avatar::DeclAvatar, controller::DeclFxController, menu::DeclSubMenu,
        parameter::DeclParameters, StaticTypeName,
    },
    sexpr::{
        argument::{flatten_args_onestep, SeparateArguments},
        error::{DeclSexprError, KetosResult},
        register_function, KetosValueExt,
    },
};

use ketos::{Arity, Error, Name, NameStore, Scope, Value};

pub fn register_avatar_function(scope: &Scope) {
    register_function(scope, "avatar", declare_avatar, Arity::Min(1), &[]);
}

fn declare_avatar(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;

    let mut avatar = DeclAvatar {
        name: name.to_string(),
        parameters_blocks: vec![],
        assets_blocks: vec![],
        fx_controllers: vec![],
        menu_blocks: vec![],
    };
    flatten_args_onestep(args.args_after(function_name, 1)?, |b| {
        match b.type_name() {
            DeclParameters::TYPE_NAME => {
                let value_ref: &DeclParameters = b.downcast_foreign_ref()?;
                avatar.parameters_blocks.push(value_ref.clone());
            }
            DeclAssets::TYPE_NAME => {
                let value_ref: &DeclAssets = b.downcast_foreign_ref()?;
                avatar.assets_blocks.push(value_ref.clone());
            }
            DeclFxController::TYPE_NAME => {
                let value_ref: &DeclFxController = b.downcast_foreign_ref()?;
                avatar.fx_controllers.push(value_ref.clone());
            }
            DeclSubMenu::TYPE_NAME => {
                let value_ref: &DeclSubMenu = b.downcast_foreign_ref()?;
                avatar.menu_blocks.push(value_ref.clone());
            }
            _ => {
                return Err(Error::Custom(
                    DeclSexprError::UnexpectedTypeValue(
                        b.type_name().to_string(),
                        "avatar element".to_string(),
                    )
                    .into(),
                ))
            }
        }
        Ok(())
    })?;

    Ok(avatar.into())
}
