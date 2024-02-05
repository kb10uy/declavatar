use crate::{
    avatar_v2::{data::avatar::Avatar, Transformer},
    decl_v2::{compile_declaration, Arguments, DeclarationFormat},
    log::{Log, SerializedLog},
};

use serde::Serialize;
use serde_json::Error as SerdeJsonError;

#[derive(Debug, Clone)]
pub struct DeclavatarState {
    args: Arguments,
}

impl DeclavatarState {
    pub fn new() -> DeclavatarState {
        DeclavatarState {
            args: Arguments::new(),
        }
    }

    pub fn arguments_mut(&mut self) -> &mut Arguments {
        &mut self.args
    }

    pub fn compile(
        &self,
        source: &str,
        format: DeclarationFormat,
    ) -> Result<CompiledState, SerdeJsonError> {
        let decl_avatar = match compile_declaration(source, format, self.args.clone()) {
            Ok(avatar) => avatar,
            Err(err) => {
                return Ok(CompiledState {
                    avatar: None,
                    logs: vec![Jsoned::new(err.serialize_log([]))?],
                });
            }
        };

        let transformer = Transformer::new();
        let transformed = transformer.transform_avatar(decl_avatar);
        Ok(CompiledState {
            avatar: transformed.avatar.map(Jsoned::new).transpose()?,
            logs: transformed
                .logs
                .into_iter()
                .map(Jsoned::new)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CompiledState {
    avatar: Option<Jsoned<Avatar>>,
    logs: Vec<Jsoned<SerializedLog>>,
}

impl CompiledState {
    pub fn avatar_json(&self) -> Option<&str> {
        self.avatar
            .as_ref()
            .map(|a| a.json().expect("should be serialized"))
    }

    pub fn logs_len(&self) -> usize {
        self.logs.len()
    }

    pub fn log_json(&self, index: usize) -> Option<&str> {
        self.logs
            .get(index)
            .map(|a| a.json().expect("should be serialized"))
    }
}

#[derive(Debug, Clone)]
struct Jsoned<T> {
    data: T,
    serialized: Option<String>,
}

impl<T: Serialize> Jsoned<T> {
    pub fn new(data: T) -> Result<Jsoned<T>, SerdeJsonError> {
        let serialized = Some(serde_json::to_string(&data)?);
        Ok(Jsoned { data, serialized })
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn json(&self) -> Option<&str> {
        self.serialized.as_deref()
    }

    pub fn update(&mut self, f: impl FnOnce(&mut T)) -> Result<(), SerdeJsonError> {
        f(&mut self.data);
        self.serialize()?;
        Ok(())
    }

    pub fn serialize(&mut self) -> Result<(), SerdeJsonError> {
        self.serialized = Some(serde_json::to_string(&self.data)?);
        Ok(())
    }
}
