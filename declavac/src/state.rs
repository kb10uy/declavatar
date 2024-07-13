use crate::{serialization::Jsoned, DeclavatarStatus};

use declavatar::{
    avatar_v2::{
        data::{attachment::schema::Attachment, avatar::Avatar},
        Transformer,
    },
    decl_v2::{compile_declaration, Arguments, DeclarationFormat},
    log::{Log, SerializedLog},
};

#[derive(Debug, Clone)]
pub struct DeclavatarState {
    args: Arguments,
    attachments: Vec<Attachment>,
    last_error: Option<String>,
}

impl DeclavatarState {
    pub fn new() -> DeclavatarState {
        DeclavatarState {
            args: Arguments::new(),
            attachments: vec![],
            last_error: None,
        }
    }

    pub fn last_error(&self) -> (Option<&str>, DeclavatarStatus) {
        (self.last_error.as_deref(), DeclavatarStatus::Success)
    }

    pub fn clear(&mut self) -> DeclavatarStatus {
        self.args.clear();
        self.attachments.clear();

        self.last_error = None;
        DeclavatarStatus::Success
    }

    pub fn add_library_path(&mut self, path: &str) -> DeclavatarStatus {
        self.args.add_library_path(path);

        self.last_error = None;
        DeclavatarStatus::Success
    }

    pub fn define_symbol(&mut self, symbol: &str) -> DeclavatarStatus {
        self.args.define_symbol(symbol);

        self.last_error = None;
        DeclavatarStatus::Success
    }

    pub fn define_localization(&mut self, key: &str, value: &str) -> DeclavatarStatus {
        self.args.define_localization(key, value);

        self.last_error = None;
        DeclavatarStatus::Success
    }

    pub fn add_attachment(&mut self, schema_json: &str) -> DeclavatarStatus {
        let schema = match serde_json::from_str::<Attachment>(schema_json) {
            Ok(schema) => schema,
            Err(err) => {
                self.last_error = Some(err.to_string());
                return DeclavatarStatus::JsonError;
            }
        };
        self.attachments.push(schema);

        self.last_error = None;
        DeclavatarStatus::Success
    }

    pub fn compile(&self, source: &str, format: DeclarationFormat) -> (CompiledState, DeclavatarStatus) {
        let decl_avatar = match compile_declaration(source, format, self.args.clone()) {
            Ok(avatar) => avatar,
            Err(err) => {
                let log = Jsoned::new(err.serialize_log([])).expect("should be serialized");
                return (
                    CompiledState {
                        avatar: None,
                        logs: vec![log],
                    },
                    DeclavatarStatus::CompileError,
                );
            }
        };

        let mut transformer = Transformer::new();
        for attachment in &self.attachments {
            transformer.register_arbittach_schema(attachment.clone());
        }
        let transformed = transformer.transform_avatar(decl_avatar);
        let avatar = transformed
            .avatar
            .map(Jsoned::new)
            .transpose()
            .expect("should be serialized");
        let logs = transformed
            .logs
            .into_iter()
            .map(Jsoned::new)
            .collect::<Result<Vec<_>, _>>()
            .expect("should be serialized");
        (CompiledState { avatar, logs }, DeclavatarStatus::Success)
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
        self.avatar.as_ref().map(|a| a.json().expect("should be serialized"))
    }

    pub fn logs_len(&self) -> usize {
        self.logs.len()
    }

    pub fn log_json(&self, index: usize) -> Option<&str> {
        self.logs.get(index).map(|a| a.json().expect("should be serialized"))
    }
}
