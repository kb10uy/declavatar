use crate::{
    avatar::{
        data::{Avatar, Driver, MenuGroup},
        transformer::{
            animations::compile_animations_blocks,
            assets::compile_assets_blocks,
            dependencies::{CompiledAnimations, CompiledSources},
            failure,
            parameters::compile_parameter_blocks,
            success, Compiled, Context, LogKind,
        },
    },
    decl::data::Menu as DeclMenu,
};

pub fn compile_menu(ctx: &mut Context, decl_menu_blocks: Vec<DeclMenu>) -> Compiled<MenuGroup> {
    failure()
}
