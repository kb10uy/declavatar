use crate::{
    avatar::{
        data::DriverGroup,
        transformer::{
            animations::compile_animations_blocks,
            assets::compile_assets_blocks,
            dependencies::{CompiledAnimations, CompiledSources},
            failure,
            parameters::compile_parameter_blocks,
            success, Compiled, Context, LogKind,
        },
    },
    decl::data::Drivers as DeclDrivers,
};

pub fn compile_drivers_blocks(
    ctx: &mut Context,
    decl_drivers_blocks: Vec<DeclDrivers>,
) -> Compiled<Vec<DriverGroup>> {
    failure()
}
