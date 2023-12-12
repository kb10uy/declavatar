use crate::{
    avatar_v2::{
        data::{avatar::Avatar, layer::Layer},
        logger::{Logger, Log, LoggerContext},
        transformer::{
            asset::compile_assets_blocks, controller::compile_fx_controller_blocks, failure,
            menu::compile_menu, parameter::compile_parameters_blocks, success, Compiled,
            CompiledAnimations, CompiledSources,
        },
    },
    decl_v2::data::avatar::DeclAvatar,
};

pub fn compile_avatar(logger: &Logger, avatar: DeclAvatar) -> Compiled<Avatar> {
    #[derive(Debug)]
    pub struct Context;
    impl LoggerContext for Context {
        fn write_context(&self, inner: String) -> String {
            inner
        }
    }

    let logger = logger.with_context(Context);

    let name = {
        let decl_name = avatar.name.trim().to_string();
        if decl_name.is_empty() {
            logger.log(Log::InvalidAvatarName(decl_name));
            return failure();
        }
        decl_name
    };

    let parameters = compile_parameters_blocks(&logger, avatar.parameters_blocks)?;
    let assets = compile_assets_blocks(&logger, avatar.assets_blocks)?;

    let compiled_sources = CompiledSources::new(&parameters, &assets);
    let fx_controller =
        compile_fx_controller_blocks(&logger, &compiled_sources, avatar.fx_controllers)?;

    let layers: Vec<&Layer> = fx_controller.iter().collect();
    let compiled_animations = CompiledAnimations::new(compiled_sources, layers);
    let menu_items = compile_menu(&logger, &compiled_animations, avatar.menu_blocks)?;

    if logger.erroneous() {
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
