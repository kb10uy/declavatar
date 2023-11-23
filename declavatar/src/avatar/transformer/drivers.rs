use crate::{
    avatar::{
        data::DriverGroup,
        transformer::{
            dependencies::{CompiledAnimations, CompiledSources},
            failure, success, Compiled, Context, LogKind,
        },
    },
    decl::data::Drivers as DeclDrivers,
};

pub fn compile_drivers_blocks(
    ctx: &mut Context,
    animations: &CompiledAnimations,
    decl_drivers_blocks: Vec<DeclDrivers>,
) -> Compiled<Vec<DriverGroup>> {
    failure()
}
