use crate::{
    avatar_v2::{
        data::avatar::Avatar,
        log::Log,
        transformer::{
            asset::compile_assets_blocks,
            controller::{compile_fx_controller_blocks, first_pass_fx_controller_blocks},
            failure,
            menu::compile_menu,
            parameter::compile_parameters_blocks,
            success, Compiled, FirstPassData,
        },
    },
    decl_v2::data::avatar::DeclAvatar,
    log::Logger,
};

pub fn compile_avatar(logger: &Logger<Log>, avatar: DeclAvatar) -> Compiled<Avatar> {
    let logger = logger.with_context("avatar");

    let name = {
        let decl_name = avatar.name.trim().to_string();
        if decl_name.is_empty() {
            logger.log(Log::InvalidAvatarName(decl_name));
            return failure();
        }
        decl_name
    };

    // first pass
    let parameters = compile_parameters_blocks(&logger, avatar.parameters_blocks)?;
    let assets = compile_assets_blocks(&logger, avatar.assets_blocks)?;
    let fx_first_pass = first_pass_fx_controller_blocks(&logger, &avatar.fx_controllers)?;
    let first_pass = FirstPassData::new(parameters, assets, fx_first_pass);

    // second pass
    let fx_controller = compile_fx_controller_blocks(&logger, &first_pass, avatar.fx_controllers)?;
    let menu_items = compile_menu(&logger, &first_pass, avatar.menu_blocks)?;

    if logger.erroneous() {
        return failure();
    }

    let (parameters, assets) = first_pass.take_back();
    success(Avatar {
        name,
        parameters,
        assets,
        fx_controller,
        menu_items,
    })
}
