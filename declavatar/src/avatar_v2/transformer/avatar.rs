use crate::{
    avatar_v2::{
        data::{avatar::Avatar, layer::Layer},
        logging::{LogKind, LoggingContext},
        transformer::{
            asset::compile_assets_blocks, controller::compile_fx_controller_blocks, failure,
            menu::compile_menu, parameter::compile_parameters_blocks, success, Compiled,
            CompiledAnimations, CompiledSources,
        },
    },
    decl_v2::data::avatar::DeclAvatar,
};

pub fn compile_avatar(ctx: &mut LoggingContext, avatar: DeclAvatar) -> Compiled<Avatar> {
    let name = {
        let decl_name = avatar.name.trim().to_string();
        if decl_name.is_empty() {
            ctx.log_error(LogKind::InvalidAvatarName(decl_name));
            return failure();
        }
        decl_name
    };

    let parameters = compile_parameters_blocks(ctx, avatar.parameters_blocks)?;
    let assets = compile_assets_blocks(ctx, avatar.assets_blocks)?;

    let compiled_sources = CompiledSources::new(&parameters, &assets);
    let fx_controller =
        compile_fx_controller_blocks(ctx, &compiled_sources, avatar.fx_controllers)?;

    let layers: Vec<&Layer> = fx_controller.iter().collect();
    let compiled_animations = CompiledAnimations::new(compiled_sources, layers);
    let menu_items = compile_menu(ctx, &compiled_animations, avatar.menu_blocks)?;

    if ctx.errornous() {
        return failure();
    }

    success(Avatar {
        name,
        parameters,
        assets,
        fx_controller,
        menu_items,
    })
}
