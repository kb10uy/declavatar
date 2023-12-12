use std::{
    cell::{Cell, RefCell},
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    rc::Rc,
};

use rpds::Stack;

type LogPair = (Log, Stack<Box<dyn LoggerContext>>);

pub trait LoggerContext: 'static + Debug {
    fn write_context(&self, inner: String) -> String;
}

#[derive(Debug)]
pub struct Logger {
    logs: Rc<RefCell<Vec<LogPair>>>,
    erroneous: Rc<Cell<bool>>,
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            logs: Rc::new(RefCell::new(vec![])),
            erroneous: Rc::new(Cell::new(false)),
        }
    }

    pub fn erroneous(&self) -> bool {
        self.erroneous.get()
    }

    pub fn logs(&self) -> Vec<(Severity, String)> {
        let logs = self.logs.borrow();
        logs.iter()
            .map(|(log, context_tail)| {
                let severity = log.severity();
                let message = context_tail
                    .iter()
                    .fold(log.to_string(), |inner, ctx| ctx.write_context(inner));
                (severity, message)
            })
            .collect()
    }

    pub(super) fn with_context<C: LoggerContext>(&self, context: C) -> ContextualLogger {
        ContextualLogger {
            logs: self.logs.clone(),
            erroneous: self.erroneous.clone(),
            context: Stack::new().push(Box::new(context)),
        }
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ContextualLogger {
    logs: Rc<RefCell<Vec<LogPair>>>,
    erroneous: Rc<Cell<bool>>,
    context: Stack<Box<dyn LoggerContext>>,
}

#[allow(dead_code)]
impl ContextualLogger {
    pub(super) fn log(&self, log: Log) {
        let mut logs = self.logs.borrow_mut();
        logs.push((log, self.context.clone()));
        self.erroneous.set(true);
    }

    pub fn erroneous(&self) -> bool {
        self.erroneous.get()
    }

    pub(super) fn with_context<C: LoggerContext>(&self, context: C) -> ContextualLogger {
        ContextualLogger {
            logs: self.logs.clone(),
            erroneous: self.erroneous.clone(),
            context: self.context.push(Box::new(context)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Severity {
    Information,
    Warning,
    Error,
}

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

    LayerNotFound(String),
    LayerMustBeGroup(String),
    LayerMustBeSwitch(String),
    LayerMustBePuppet(String),
    LayerOptionNotFound(String),
    LayerDisabledTargetFailure(String),
    LayerMaterialFailure(usize),
    LayerIndeterminateShapeChange(String),
    LayerIndeterminateMaterialChange(usize),
    LayerIndeterminateOption(String, String),
    LayerStateNotFound(String, String),
    LayerBlendTreeParameterNotFound(String, String),

    DriverOptionNotSpecified(String),
    DriverInvalidAddTarget(String),
    DriverInvalidRandomSpecification(String),
    DriverInvalidCopyTarget(String),
    DriverCopyMismatch(String, (String, String)),
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

            Log::LayerNotFound(group) => {
                write!(f, "layer '{group}' not found")
            }
            Log::LayerMustBeGroup(group) => {
                write!(f, "layer '{group}' must be group")
            }
            Log::LayerMustBeSwitch(group) => {
                write!(f, "layer '{group}' must be switch")
            }
            Log::LayerMustBePuppet(group) => {
                write!(f, "layer '{group}' must be puppet")
            }
            Log::LayerOptionNotFound(option) => {
                write!(f, "option '{option}' not found")
            }
            Log::LayerDisabledTargetFailure(target) => {
                write!(f, "target '{target}' has no auto-generated disabled target")
            }
            Log::LayerMaterialFailure(group) => {
                write!(f, "group name '{group}', material target failure")
            }
            Log::LayerIndeterminateShapeChange(shape) => {
                write!(f, "'{shape}' does not have mesh target")
            }
            Log::LayerIndeterminateMaterialChange(material) => {
                write!(f, "'{material}' does not have mesh target")
            }
            Log::LayerIndeterminateOption(group, option) => {
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
