pub mod data;
pub mod log;
mod transformer;

use crate::{
    avatar_v2::{
        data::{attachment::schema::Attachment, avatar::Avatar},
        transformer::compile_avatar,
    },
    decl_v2::data::avatar::DeclAvatar,
    log::{Logger, SerializedLog},
};

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Transformer {
    arbittach_schemas: HashMap<String, Attachment>,
}

impl Transformer {
    pub fn new() -> Transformer {
        Transformer {
            arbittach_schemas: HashMap::new(),
        }
    }

    pub fn register_arbittach_schema(&mut self, schema: Attachment) {
        let name = schema.name.clone();
        self.arbittach_schemas.insert(name, schema);
    }

    pub fn transform_avatar(&self, avatar: DeclAvatar) -> TransformResult {
        let logger = Logger::new();
        let avatar = compile_avatar(&logger, avatar, &self.arbittach_schemas);
        let logs = logger.serialize_logs();

        TransformResult { avatar, logs }
    }
}

pub struct TransformResult {
    pub avatar: Option<Avatar>,
    pub logs: Vec<SerializedLog>,
}
