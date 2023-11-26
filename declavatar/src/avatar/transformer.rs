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

    LayerStateNotFound(String, String),
    LayerBlendTreeParameterNotFound(String, String),
}

impl Display for LogKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            LogKind::InvalidAvatarName(name) => write!(f, "invalid avatar name '{name}'"),
            LogKind::InternalMustBeTransient(param) => {
                write!(f, "internal parameter '{param}' cannot be saved")
            }
            LogKind::IncompatibleParameterDeclaration(param) => {
                write!(f, "parameter '{param}' has incompatible declarations")
            }
            LogKind::IndeterminateAsset(asset) => write!(f, "indeterminate asset '{asset}'"),
            LogKind::IncompatibleAssetDeclaration(asset) => {
                write!(f, "asset '{asset}' has incompatible declaration")
            }
            LogKind::DuplicateGroupName(group) => write!(f, "group name '{group}' is duplicate"),

            LogKind::ParameterNotFound(param) => write!(f, "parameter '{param}' not found"),
            LogKind::ParameterTypeRequirement(param, ty) => {
                write!(f, "parameter '{param}' must be {ty:?}")
            }
            LogKind::ParameterScopeRequirement(param, scope) => {
                write!(f, "parameter '{param}' must be {scope:?}")
            }

            LogKind::AssetNotFound(asset) => write!(f, "asset '{asset}' not found"),
            LogKind::AssetTypeRequirement(asset, ty) => write!(f, "asset '{asset}' must be {ty:?}"),

            LogKind::AnimationGroupNotFound(group) => {
                write!(f, "animation group '{group}' not found")
            }
            LogKind::AnimationGroupMustBeGroup(group) => {
                write!(f, "group '{group}' must be group block")
            }
            LogKind::AnimationGroupMustBeSwitch(group) => {
                write!(f, "group '{group}' must be switch block")
            }
            LogKind::AnimationGroupMustBePuppet(group) => {
                write!(f, "group '{group}' must be puppet block")
            }
            LogKind::AnimationGroupOptionNotFound(group, option) => {
                write!(f, "group '{group}', option '{option}' not found")
            }
            LogKind::AnimationGroupDisabledTargetFailure(group, target) => {
                write!(
                    f,
                    "group name '{group}', target '{target}' has no auto-generated disabled target"
                )
            }
            LogKind::AnimationGroupMaterialFailure(group) => {
                write!(f, "group name '{group}', material target failure")
            }
            LogKind::AnimationGroupIndeterminateShapeChange(group, shape) => {
                write!(
                    f,
                    "group name '{group}', '{shape}' does not have mesh target"
                )
            }
            LogKind::AnimationGroupIndeterminateMaterialChange(group, material) => {
                write!(
                    f,
                    "group name '{group}', '{material}' does not have mesh target"
                )
            }
            LogKind::AnimationGroupIndeterminateOption(group, option) => {
                write!(f, "group name '{group}', option '{option}' not found")
            }

            LogKind::DriverOptionNotSpecified(driver) => {
                write!(f, "driver option '{driver}' not specified")
            }
            LogKind::DriverInvalidAddTarget(driver) => {
                write!(f, "driver '{driver}' has invalid add target")
            }
            LogKind::DriverInvalidRandomSpecification(driver) => {
                write!(
                    f,
                    "driver '{driver}' has invalid random target specification"
                )
            }
            LogKind::DriverInvalidCopyTarget(driver) => {
                write!(f, "driver option '{driver}' has invalid copy target")
            }
            LogKind::DriverCopyMismatch(driver, (from, to)) => {
                write!(f, "driver option '{driver}' has copy target; parameters '{from}' and '{to}' have different type")
            }

            LogKind::LayerStateNotFound(layer, state) => {
                write!(f, "layer '{layer}', state '{state}' not found")
            }
            LogKind::LayerBlendTreeParameterNotFound(layer, state) => write!(
                f,
                "layer '{layer}', state '{state}' does not sufficient parameters"
            ),
        }
    }
}