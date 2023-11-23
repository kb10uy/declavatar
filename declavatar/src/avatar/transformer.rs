mod animations;
mod assets;
mod avatar;
mod dependencies;
mod drivers;
mod menu;
mod parameters;

// Renamed for future change
pub type Compiled<T> = Option<T>;

// Reserved as a function for future change
#[inline]
pub fn success<T>(t: T) -> Compiled<T> {
    Some(t)
}

// Reserved as a function for future change
#[inline]
pub fn failure<T>() -> Compiled<T> {
    None
}

#[derive(Debug)]
struct Context {
    logs: Vec<(LogLevel, LogKind)>,
    errornous: bool,
}

impl Context {
    pub fn new() -> Context {
        Context {
            logs: vec![],
            errornous: false,
        }
    }

    pub fn log_info(&mut self, log: LogKind) {
        self.logs.push((LogLevel::Information, log));
    }

    pub fn log_warn(&mut self, log: LogKind) {
        self.logs.push((LogLevel::Warning, log));
    }

    pub fn log_error(&mut self, log: LogKind) {
        self.logs.push((LogLevel::Error, log));
        self.errornous = true;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogLevel {
    Information,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LogKind {
    InvalidAvatarName(String),
    InternalMustBeTransient(String),
    IncompatibleParameterDeclaration(String),
    IndeterminateAsset(String),
    IncompatibleAssetDeclaration(String),
    DuplicateGroupName(String),

    ParameterNotFound(String),
}
