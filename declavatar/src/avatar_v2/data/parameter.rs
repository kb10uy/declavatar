use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Parameter {
    pub name: String,
    pub value_type: ParameterType,
    pub scope: ParameterScope,
    pub unique: bool,
    pub explicit_default: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(tag = "type", content = "default")]
pub enum ParameterType {
    Int(u8),
    Float(f64),
    Bool(bool),
}

impl ParameterType {
    pub const INT_TYPE: ParameterType = ParameterType::Int(0);
    pub const FLOAT_TYPE: ParameterType = ParameterType::Float(0.0);
    pub const BOOL_TYPE: ParameterType = ParameterType::Bool(false);

    pub fn matches(self, requirement: ParameterType) -> bool {
        matches!(
            (self, requirement),
            (ParameterType::Int(_), ParameterType::Int(_))
                | (ParameterType::Float(_), ParameterType::Float(_))
                | (ParameterType::Bool(_), ParameterType::Bool(_))
        )
    }

    pub const fn type_name(self) -> &'static str {
        match self {
            ParameterType::Int(_) => "int",
            ParameterType::Float(_) => "float",
            ParameterType::Bool(_) => "bool",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(tag = "type", content = "save")]
pub enum ParameterScope {
    Internal,
    Local(bool),
    Synced(bool),
}

impl ParameterScope {
    pub const MAYBE_INTERNAL: ParameterScope = ParameterScope::Internal;
    pub const MUST_EXPOSE: ParameterScope = ParameterScope::Local(false);
    pub const MUST_SYNC: ParameterScope = ParameterScope::Synced(false);

    pub const fn suitable_for(self, requirement: ParameterScope) -> bool {
        matches!(
            (requirement, self),
            (ParameterScope::Internal, _)
                | (
                    ParameterScope::Local(_),
                    ParameterScope::Local(_) | ParameterScope::Synced(_)
                )
                | (ParameterScope::Synced(_), ParameterScope::Synced(_))
        )
    }

    pub const fn name(self) -> &'static str {
        match self {
            ParameterScope::Internal => "internal",
            ParameterScope::Local(_) => "local",
            ParameterScope::Synced(_) => "synced",
        }
    }
}
