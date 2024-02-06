use crate::serialization::Jsoned;

use declavatar::{
    avatar_v2::{
        data::{attachment::schema::Attachment, avatar::Avatar},
        Transformer,
    },
    decl_v2::{compile_declaration, Arguments, DeclarationFormat},
    log::{Log, SerializedLog},
};
use serde_json::Error as SerdeJsonError;

#[derive(Debug, Clone)]
pub struct DeclavatarState {
    args: Arguments,
    attachments: Vec<Attachment>,
}

impl DeclavatarState {
    pub fn new() -> DeclavatarState {
        DeclavatarState {
            args: Arguments::new(),
            attachments: vec![],
        }
    }

    pub fn arguments_mut(&mut self) -> &mut Arguments {
        &mut self.args
    }

    pub fn add_attachment(&mut self, attachment: Attachment) {
        self.attachments.push(attachment);
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

        let mut transformer = Transformer::new();
        for attachment in &self.attachments {
            transformer.register_arbittach_schema(attachment.clone());
        }
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

impl Default for DeclavatarState {
    fn default() -> Self {
        Self::new()
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
