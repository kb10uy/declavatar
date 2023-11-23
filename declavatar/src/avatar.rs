pub mod data;
pub mod error;

mod compiler;
mod transformer;

use crate::{
    avatar::{
        compiler::AvatarCompiler,
        data::Avatar,
        error::{AvatarError, Result},
        transformer::{compile_avatar as transformer_compile_avatar, Context},
    },
    compiler::Compiler,
    decl::data::Avatar as DeclAvatar,
};

use std::result::Result as StdResult;

pub struct TransformResult {
    pub avatar: Option<Avatar>,
    pub logs: Vec<(String, String)>,
}

pub fn transform_avatar(avatar: DeclAvatar) -> TransformResult {
    let mut ctx = Context::new();
    let avatar = transformer_compile_avatar(&mut ctx, avatar);
    let logs = ctx.into_logs();

    TransformResult { avatar, logs }
}

#[deprecated = "Use transform_avatar"]
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
