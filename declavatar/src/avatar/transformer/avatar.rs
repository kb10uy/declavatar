use crate::{
    avatar::{
        data::Avatar,
        transformer::{
            animations::compile_animations_blocks,
            assets::compile_assets_blocks,
            dependencies::{CompiledAnimations, CompiledSources},
            drivers::compile_drivers_blocks,
            failure,
            menu::compile_menu,
            parameters::compile_parameter_blocks,
            success, Compiled, Context, LogKind,
        },
    },
    decl::data::Avatar as DeclAvatar,
};

pub fn compile_avatar(ctx: &mut Context, avatar: DeclAvatar) -> Compiled<Avatar> {
    let name = {
        let decl_name = avatar.name.trim().to_string();
        if decl_name.is_empty() {
            ctx.log_error(LogKind::InvalidAvatarName(decl_name));
            return failure();
        }
        decl_name
    };

    let parameters = compile_parameter_blocks(ctx, avatar.parameters_blocks)?;
    let assets = compile_assets_blocks(ctx, avatar.assets_blocks)?;
    let compiled_sources = CompiledSources::new(parameters, assets);

    let animation_groups =
        compile_animations_blocks(ctx, &compiled_sources, avatar.animations_blocks)?;
    let compiled_animations = CompiledAnimations::new(compiled_sources, animation_groups);

    let driver_groups = compile_drivers_blocks(ctx, avatar.drivers_blocks)?;
    let top_menu_group = compile_menu(ctx, avatar.menu_blocks)?;

    let (parameters, assets, animation_groups) = compiled_animations.deconstruct();
    success(Avatar {
        name,
        parameters,
        assets,
        animation_groups,
        driver_groups,
        top_menu_group,
    })
}
