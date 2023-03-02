pub mod data;
pub mod error;

mod compiler;

use crate::{
    avatar::{
        compiler::AvatarCompiler,
        data::Avatar,
        error::{AvatarError, Result},
    },
    compiler::Compiler,
    decl::data::Avatar as DeclAvatar,
};

use std::result::Result as StdResult;

pub fn compile_avatar(avatar: DeclAvatar) -> Result<StdResult<Avatar, Vec<String>>> {
    let mut compiler = AvatarCompiler::new();
    let compiled_avatar = compiler.parse(avatar)?;

    if compiler.errornous() {
        Ok(Err(compiler
            .messages()
            .into_iter()
            .map(|(_, m)| m)
            .collect()))
    } else if let Some(a) = compiled_avatar {
        Ok(Ok(a))
    } else {
        Err(AvatarError::CompilerError(
            "neither functional avatar nor error list has been generated".into(),
        ))
    }
}
