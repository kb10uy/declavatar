use std::fmt::{Debug, Display, Error as FmtError, Formatter, Result as FmtResult};

use rpds::Stack;

#[derive(Debug)]
pub struct Logger {
    logs: Vec<(Log, Stack<Box<dyn LoggerContext>>)>,
    context: Stack<Box<dyn LoggerContext>>,
    erroneous: bool,
}

#[allow(unused_variables)]
pub trait LoggerContext: 'static + Debug {
    fn write_context(&self, inner: String) -> String;
}

#[allow(dead_code)]
impl Logger {
    pub fn new() -> Logger {
        Logger {
            logs: vec![],
            context: Stack::new(),
            erroneous: false,
        }
    }

    pub(super) fn push_context<C: LoggerContext>(&mut self, context: C) {
        self.context = self.context.push(Box::new(context));
    }

    pub(super) fn pop_context(&mut self) {
        self.context = self.context.pop().expect("too much pops");
    }

    pub(super) fn erroneous(&self) -> bool {
        self.erroneous
    }

    pub fn into_logs(&self) -> Result<Vec<(Severity, String)>, FmtError> {
        let mut logs = vec![];
        for (log, context_tail) in &self.logs {
            let severity = log.severity();
            let message = context_tail
                .iter()
                .fold(log.to_string(), |inner, ctx| ctx.write_context(inner));
            logs.push((severity, message));
        }

        Ok(logs)
    }

    pub(super) fn log(&mut self, log: Log) {
        self.logs.push((log, self.context.clone()));
        self.erroneous = true;
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Severity {
    Information,
    Warning,
    Error,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Log {
    InvalidAvatarName(String),
    InternalMustBeTransient(String),
    IncompatibleParameterDeclaration(String),
    IndeterminateAsset(String),
    IncompatibleAssetDeclaration(String),
    DuplicateLayerName(String),

    ParameterNotFound(String),
    ParameterTypeRequirement(String, String),
    ParameterScopeRequirement(String, String),

    AssetNotFound(String),
    AssetTypeRequirement(String, String),

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

impl Log {
    pub fn severity(&self) -> Severity {
        Severity::Error
    }
}

impl Display for Log {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Log::InvalidAvatarName(name) => write!(f, "invalid avatar name '{name}'"),
            Log::InternalMustBeTransient(param) => {
                write!(f, "internal parameter '{param}' cannot be saved")
            }
            Log::IncompatibleParameterDeclaration(param) => {
                write!(f, "parameter '{param}' has incompatible declarations")
            }
            Log::IndeterminateAsset(asset) => write!(f, "indeterminate asset '{asset}'"),
            Log::IncompatibleAssetDeclaration(asset) => {
                write!(f, "asset '{asset}' has incompatible declaration")
            }
            Log::DuplicateLayerName(group) => write!(f, "layer name '{group}' is duplicate"),

            Log::ParameterNotFound(param) => write!(f, "parameter '{param}' not found"),
            Log::ParameterTypeRequirement(param, ty) => {
                write!(f, "parameter '{param}' must be {ty}")
            }
            Log::ParameterScopeRequirement(param, scope) => {
                write!(f, "parameter '{param}' must be {scope}")
            }

            Log::AssetNotFound(asset) => write!(f, "asset '{asset}' not found"),
            Log::AssetTypeRequirement(asset, ty) => write!(f, "asset '{asset}' must be {ty}"),

            Log::AnimationGroupNotFound(group) => {
                write!(f, "animation group '{group}' not found")
            }
            Log::AnimationGroupMustBeGroup(group) => {
                write!(f, "group '{group}' must be group block")
            }
            Log::AnimationGroupMustBeSwitch(group) => {
                write!(f, "group '{group}' must be switch block")
            }
            Log::AnimationGroupMustBePuppet(group) => {
                write!(f, "group '{group}' must be puppet block")
            }
            Log::AnimationGroupOptionNotFound(group, option) => {
                write!(f, "group '{group}', option '{option}' not found")
            }
            Log::AnimationGroupDisabledTargetFailure(group, target) => {
                write!(
                    f,
                    "group name '{group}', target '{target}' has no auto-generated disabled target"
                )
            }
            Log::AnimationGroupMaterialFailure(group) => {
                write!(f, "group name '{group}', material target failure")
            }
            Log::AnimationGroupIndeterminateShapeChange(group, shape) => {
                write!(
                    f,
                    "group name '{group}', '{shape}' does not have mesh target"
                )
            }
            Log::AnimationGroupIndeterminateMaterialChange(group, material) => {
                write!(
                    f,
                    "group name '{group}', '{material}' does not have mesh target"
                )
            }
            Log::AnimationGroupIndeterminateOption(group, option) => {
                write!(f, "group name '{group}', option '{option}' not found")
            }

            Log::DriverOptionNotSpecified(driver) => {
                write!(f, "driver option '{driver}' not specified")
            }
            Log::DriverInvalidAddTarget(driver) => {
                write!(f, "driver '{driver}' has invalid add target")
            }
            Log::DriverInvalidRandomSpecification(driver) => {
                write!(
                    f,
                    "driver '{driver}' has invalid random target specification"
                )
            }
            Log::DriverInvalidCopyTarget(driver) => {
                write!(f, "driver option '{driver}' has invalid copy target")
            }
            Log::DriverCopyMismatch(driver, (from, to)) => {
                write!(f, "driver option '{driver}' has copy target; parameters '{from}' and '{to}' have different type")
            }

            Log::LayerStateNotFound(layer, state) => {
                write!(f, "layer '{layer}', state '{state}' not found")
            }
            Log::LayerBlendTreeParameterNotFound(layer, state) => write!(
                f,
                "layer '{layer}', state '{state}' does not sufficient parameters"
            ),
        }
    }
}
