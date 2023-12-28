use std::{
    cell::{Cell, RefCell},
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    rc::Rc,
};

use rpds::Stack;
use serde::Serialize;

type ContextChain = Stack<Box<dyn Context>>;

#[derive(Debug)]
pub struct Logger<L> {
    logs: Rc<RefCell<Vec<(L, ContextChain)>>>,
    erroneous: Rc<Cell<bool>>,
    context: ContextChain,
}

impl<L: Log> Logger<L> {
    pub fn new() -> Logger<L> {
        Logger {
            logs: Rc::new(RefCell::new(vec![])),
            erroneous: Rc::new(Cell::new(false)),
            context: Stack::new(),
        }
    }

    pub fn log(&self, log: L) {
        let mut logs = self.logs.borrow_mut();
        self.erroneous.set(self.erroneous.get() || log.erroneous());
        logs.push((log, self.context.clone()));
    }

    pub fn with_context<C: Context>(&self, ctx: C) -> Logger<L> {
        Logger {
            logs: self.logs.clone(),
            erroneous: self.erroneous.clone(),
            context: self.context.push(Box::new(ctx)),
        }
    }

    pub fn erroneous(&self) -> bool {
        self.erroneous.get()
    }

    pub fn serialize_logs(&self) -> Vec<SerializedLog> {
        self.logs
            .borrow()
            .iter()
            .map(|(l, c)| l.serialize_log(c.iter().map(|v| v.as_ref())))
            .collect()
    }
}

impl<L: Log> Default for Logger<L> {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Log {
    fn erroneous(&self) -> bool;
    fn serialize_log<'a, C: IntoIterator<Item = &'a dyn Context>>(
        &self,
        context: C,
    ) -> SerializedLog;
}

impl Log for String {
    fn erroneous(&self) -> bool {
        true
    }

    fn serialize_log<'a, C: IntoIterator<Item = &'a dyn Context>>(
        &self,
        context: C,
    ) -> SerializedLog {
        SerializedLog {
            severity: Severity::Error,
            kind: "".into(),
            args: vec![],
            context: context.into_iter().map(|c| c.to_string()).collect(),
        }
    }
}

pub trait Context: Debug + Display + 'static {}

impl<T: Debug + Display + 'static> Context for T {}

pub struct FormatterContext<F> {
    inner: F,
}

impl<F: Fn(&mut Formatter<'_>) -> FmtResult + 'static> FormatterContext<F> {
    pub fn new(f: F) -> FormatterContext<F> {
        FormatterContext { inner: f }
    }
}

impl<F: Fn(&mut Formatter<'_>) -> FmtResult + 'static> Debug for FormatterContext<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "FormatterContext(")?;
        (self.inner)(f)?;
        write!(f, ")")?;
        Ok(())
    }
}

impl<F: Fn(&mut Formatter<'_>) -> FmtResult + 'static> Display for FormatterContext<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        (self.inner)(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum Severity {
    Information,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SerializedLog {
    pub severity: Severity,
    pub kind: String,
    pub args: Vec<String>,
    pub context: Vec<String>,
}
