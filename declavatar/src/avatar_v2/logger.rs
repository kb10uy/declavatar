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

pub struct Logger {
    logs: Rc<RefCell<Vec<LogPair>>>,
    erroneous: Rc<Cell<bool>>,
    context: Stack<Box<dyn LoggerContext>>,
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            logs: Rc::new(RefCell::new(vec![])),
            erroneous: Rc::new(Cell::new(false)),
            context: Stack::new(),
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

    pub(super) fn log(&self, log: Log) {
        let mut logs = self.logs.borrow_mut();
        logs.push((log, self.context.clone()));
        self.erroneous.set(true);
    }

    pub(super) fn with_context<C: LoggerContext>(&self, context: C) -> Logger {
        Logger {
            logs: self.logs.clone(),
            erroneous: self.erroneous.clone(),
            context: self.context.push(Box::new(context)),
        }
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
    LayerMustBeRaw(String),
    LayerOptionNotFound(String),
    LayerOptionMustBeExclusive,
    LayerGroupInvalidCopy,
    LayerDisabledTargetFailure(String),
    LayerMaterialFailure(usize),
    LayerIndeterminateShapeChange(String),
    LayerIndeterminateMaterialChange(usize),
    LayerPuppetCannotDrive,
    LayerPuppetOptionMustBeInlined,
    LayerKeyframeOutOfRange(f64),
    LayerStateNotFound(String),
    LayerBlendTreeParameterNotFound(String),
    LayerInvalidCondition,

    MenuInvalidDrive,

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
            Log::LayerMustBeRaw(group) => {
                write!(f, "layer '{group}' must be raw")
            }
            Log::LayerOptionNotFound(option) => {
                write!(f, "option '{option}' not found")
            }
            Log::LayerOptionMustBeExclusive => {
                write!(
                    f,
                    "external animation asset cannot be combined with targets"
                )
            }
            Log::LayerGroupInvalidCopy => {
                write!(f, "group copy mode is invalid")
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
            Log::LayerPuppetCannotDrive => {
                write!(f, "puppet layer cannot drive parameters")
            }
            Log::LayerPuppetOptionMustBeInlined => {
                write!(f, "puppet option cannot be external animation")
            }
            Log::LayerKeyframeOutOfRange(value) => {
                write!(f, "puppet layer value out of range: {value}")
            }
            Log::LayerInvalidCondition => {
                write!(f, "transition condition is invalid")
            }

            Log::MenuInvalidDrive => {
                write!(f, "menu drive must be set or drive")
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

            Log::LayerStateNotFound(state) => {
                write!(f, "state '{state}' not found")
            }
            Log::LayerBlendTreeParameterNotFound(state) => {
                write!(f, "state '{state}' does not sufficient parameters")
            }
        }
    }
}
