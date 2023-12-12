use crate::{
    avatar_v2::{
        data::{avatar::Avatar, layer::Layer},
        logger::{Log, Logger, LoggerContext},
        transformer::{
            asset::compile_assets_blocks, controller::compile_fx_controller_blocks, failure,
            menu::compile_menu, parameter::compile_parameters_blocks, success, Compiled,
            CompiledAnimations, CompiledSources,
        },
    },
    decl_v2::data::avatar::DeclAvatar,
};

pub fn compile_avatar(ctx: &mut Logger, avatar: DeclAvatar) -> Compiled<Avatar> {
    #[derive(Debug)]
    pub struct Context(String);
    impl LoggerContext for Context {
        fn write_context(&self, inner: String) -> String {
            format!("avatar '{}' > {}", self.0, inner)
        }
    }

    let name = {
        let decl_name = avatar.name.trim().to_string();
        if decl_name.is_empty() {
            ctx.log(Log::InvalidAvatarName(decl_name));
            return failure();
        }
        decl_name
    };
    ctx.push_context(Context(name.clone()));

    let parameters = compile_parameters_blocks(ctx, avatar.parameters_blocks)?;
    let assets = compile_assets_blocks(ctx, avatar.assets_blocks)?;

    let compiled_sources = CompiledSources::new(&parameters, &assets);
    let fx_controller =
        compile_fx_controller_blocks(ctx, &compiled_sources, avatar.fx_controllers)?;

    let layers: Vec<&Layer> = fx_controller.iter().collect();
    let compiled_animations = CompiledAnimations::new(compiled_sources, layers);
    let menu_items = compile_menu(ctx, &compiled_animations, avatar.menu_blocks)?;

    ctx.pop_context();
    if ctx.erroneous() {
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
