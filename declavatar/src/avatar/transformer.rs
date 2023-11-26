mod animations;
mod assets;
mod avatar;
mod dependencies;
mod drivers;
mod menu;
mod parameters;

pub(super) use self::avatar::compile_avatar;

use crate::avatar::data::{AssetType, ParameterScope, ParameterType};

use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub(super) struct Context {
    logs: Vec<(LogLevel, LogKind)>,
    errornous: bool,
}

#[allow(dead_code)]
impl Context {
    pub fn new() -> Context {
        Context {
            logs: vec![],
            errornous: false,
        }
    }

    pub(self) fn log_info(&mut self, log: LogKind) {
        self.logs.push((LogLevel::Information, log));
    }

    pub(self) fn log_warn(&mut self, log: LogKind) {
        self.logs.push((LogLevel::Warning, log));
    }

    pub(self) fn log_error(&mut self, log: LogKind) {
        self.logs.push((LogLevel::Error, log));
        self.errornous = true;
    }

    pub(self) fn errornous(&self) -> bool {
        self.errornous
    }

    pub fn into_logs(self) -> Vec<(LogLevel, String)> {
        self.logs
            .into_iter()
            .map(|(ll, lk)| (ll, lk.to_string()))
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogLevel {
    Information,
    Warning,
    Error,
}

// Renamed for future change
type Compiled<T> = Option<T>;

// Reserved as a function for future change
#[inline]
fn success<T>(t: T) -> Compiled<T> {
    Some(t)
}

// Reserved as a function for future change
#[inline]
fn failure<T>() -> Compiled<T> {
    None
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum LogKind {
    InvalidAvatarName(String),
    InternalMustBeTransient(String),
    IncompatibleParameterDeclaration(String),
    IndeterminateAsset(String),
    IncompatibleAssetDeclaration(String),
    DuplicateGroupName(String),

    ParameterNotFound(String),
    ParameterTypeRequirement(String, ParameterType),
    ParameterScopeRequirement(String, ParameterScope),

    AssetNotFound(String),
    AssetTypeRequirement(String, AssetType),

    AnimationGroupNotFound(String),
    AnimationGroupMustBeGroup(String),
    AnimationGroupMustBeSwitch(String),
    AnimationGroupMustBePuppet(String),
    AnimationGroupOptionNotFound(String, String),
    AnimationGroupDisabledTargetFailure(String, String),
    AnimationGroupMaterialFailure(usize),
    AnimationGroupIndeterminateShapeChange(String, String),
    AnimationGroupIndeterminateMaterialChange(String, usize),
    AnimationGroupIndeterminateOption(String, String),

    DriverOptionNotSpecified(String),
    DriverInvalidAddTarget(String),
    DriverInvalidRandomSpecification(String),
    DriverInvalidCopyTarget(String),
    DriverCopyMismatch(String, (String, String)),
}

impl Display for LogKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            LogKind::InvalidAvatarName(name) => write!(f, "invalid avatar name '{name}'"),
            LogKind::InternalMustBeTransient(_) => todo!(),
            LogKind::IncompatibleParameterDeclaration(_) => todo!(),
            LogKind::IndeterminateAsset(_) => todo!(),
            LogKind::IncompatibleAssetDeclaration(_) => todo!(),
            LogKind::DuplicateGroupName(_) => todo!(),

            LogKind::ParameterNotFound(_) => todo!(),
            LogKind::ParameterTypeRequirement(_, _) => todo!(),
            LogKind::ParameterScopeRequirement(_, _) => todo!(),

            LogKind::AssetNotFound(_) => todo!(),
            LogKind::AssetTypeRequirement(_, _) => todo!(),

            LogKind::AnimationGroupNotFound(_) => todo!(),
            LogKind::AnimationGroupMustBeGroup(_) => todo!(),
            LogKind::AnimationGroupMustBeSwitch(_) => todo!(),
            LogKind::AnimationGroupMustBePuppet(_) => todo!(),
            LogKind::AnimationGroupOptionNotFound(_, _) => todo!(),
            LogKind::AnimationGroupDisabledTargetFailure(_, _) => todo!(),
            LogKind::AnimationGroupMaterialFailure(_) => todo!(),
            LogKind::AnimationGroupIndeterminateShapeChange(_, _) => todo!(),
            LogKind::AnimationGroupIndeterminateMaterialChange(_, _) => todo!(),
            LogKind::AnimationGroupIndeterminateOption(_, _) => todo!(),

            LogKind::DriverOptionNotSpecified(_) => todo!(),
            LogKind::DriverInvalidAddTarget(_) => todo!(),
            LogKind::DriverInvalidRandomSpecification(_) => todo!(),
            LogKind::DriverInvalidCopyTarget(_) => todo!(),
            LogKind::DriverCopyMismatch(_, _) => todo!(),
        }
    }
}
