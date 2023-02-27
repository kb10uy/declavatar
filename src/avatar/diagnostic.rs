use std::convert::Infallible;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Level {
    Information,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub level: Level,
    pub instrument: String,
    pub message: String,
    pub hint: Option<String>,
}

impl Message {
    fn info(instrument: String, message: String) -> Message {
        Message {
            level: Level::Information,
            instrument,
            message,
            hint: None,
        }
    }

    fn warn(instrument: String, message: String) -> Message {
        Message {
            level: Level::Warning,
            instrument,
            message,
            hint: None,
        }
    }

    fn error(instrument: String, message: String) -> Message {
        Message {
            level: Level::Error,
            instrument,
            message,
            hint: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InstrumentKey {
    Unkeyed,
    Array(usize),
    Map(String),
}

pub trait Diagnostic<'a>
where
    Self: 'a,
{
    type Scoped: Diagnostic<'a>;

    fn info<I: Instrument>(&mut self, key: InstrumentKey, message: String);
    fn warn<I: Instrument>(&mut self, key: InstrumentKey, message: String);
    fn error<I: Instrument>(
        &mut self,
        key: InstrumentKey,
        message: String,
    ) -> Result<Infallible, ()>;
    fn with<I: Instrument>(&'a mut self, key: InstrumentKey) -> Self::Scoped;
    fn append(&mut self, messages: Vec<Message>);
    fn commit(self);
}

#[derive(Debug, Clone)]
pub struct CompilerInfo {
    messages: Vec<Message>,
    errornous: bool,
}

impl CompilerInfo {
    pub fn new() -> CompilerInfo {
        CompilerInfo {
            messages: vec![],
            errornous: false,
        }
    }

    pub fn errornous(&self) -> bool {
        self.errornous
    }
}

impl<'a> Diagnostic<'a> for CompilerInfo {
    type Scoped = ScopedCompilerInfo<'a, CompilerInfo>;

    fn info<I: Instrument>(&mut self, key: InstrumentKey, message: String) {
        self.messages.push(Message::info(
            construct_instrument(None, I::INSTRUMENT_NAME, key),
            message,
        ));
    }

    fn warn<I: Instrument>(&mut self, key: InstrumentKey, message: String) {
        self.messages.push(Message::warn(
            construct_instrument(None, I::INSTRUMENT_NAME, key),
            message,
        ));
    }

    fn error<I: Instrument>(
        &mut self,
        key: InstrumentKey,
        message: String,
    ) -> Result<Infallible, ()> {
        self.messages.push(Message::error(
            construct_instrument(None, I::INSTRUMENT_NAME, key),
            message,
        ));
        self.errornous = true;
        Err(())
    }

    fn with<I: Instrument>(
        &'a mut self,
        key: InstrumentKey,
    ) -> ScopedCompilerInfo<'a, CompilerInfo> {
        ScopedCompilerInfo {
            parent: self,
            prefix: construct_instrument(None, I::INSTRUMENT_NAME, key),
            messages: vec![],
            errornous: false,
        }
    }

    fn append(&mut self, mut messages: Vec<Message>) {
        self.errornous = self.errornous | messages.iter().any(|m| m.level == Level::Error);
        self.messages.append(&mut messages);
    }

    fn commit(self) {}
}

pub struct ScopedCompilerInfo<'p, P> {
    parent: &'p mut P,
    prefix: String,
    messages: Vec<Message>,
    errornous: bool,
}

impl<'p, P: Diagnostic<'p>> ScopedCompilerInfo<'p, P> {
    pub fn errornous(&self) -> bool {
        self.errornous
    }

    pub fn of<'a: 'p, I: Instrument>(
        &'a mut self,
        key: InstrumentKey,
    ) -> ScopedCompilerInfo<'a, ScopedCompilerInfo<'a, P>> {
        let prefix = construct_instrument(Some(&self.prefix), I::INSTRUMENT_NAME, key);
        ScopedCompilerInfo {
            parent: self,
            prefix,
            messages: vec![],
            errornous: false,
        }
    }
}

impl<'p, P: Diagnostic<'p>> Diagnostic<'p> for ScopedCompilerInfo<'p, P> {
    type Scoped = ScopedCompilerInfo<'p, ScopedCompilerInfo<'p, P>>;

    fn info<I: Instrument>(&mut self, key: InstrumentKey, message: String) {
        self.messages.push(Message::info(
            construct_instrument(Some(&self.prefix), I::INSTRUMENT_NAME, key),
            message,
        ));
    }

    fn warn<I: Instrument>(&mut self, key: InstrumentKey, message: String) {
        self.messages.push(Message::warn(
            construct_instrument(Some(&self.prefix), I::INSTRUMENT_NAME, key),
            message,
        ));
    }

    fn error<I: Instrument>(
        &mut self,
        key: InstrumentKey,
        message: String,
    ) -> Result<Infallible, ()> {
        self.messages.push(Message::error(
            construct_instrument(Some(&self.prefix), I::INSTRUMENT_NAME, key),
            message,
        ));
        self.errornous = true;
        Err(())
    }

    fn with<I: Instrument>(
        &'p mut self,
        key: InstrumentKey,
    ) -> ScopedCompilerInfo<'p, ScopedCompilerInfo<'p, P>> {
        ScopedCompilerInfo {
            parent: self,
            prefix: construct_instrument(None, I::INSTRUMENT_NAME, key),
            messages: vec![],
            errornous: false,
        }
    }

    fn append(&mut self, mut messages: Vec<Message>) {
        self.errornous = self.errornous | messages.iter().any(|m| m.level == Level::Error);
        self.messages.append(&mut messages);
    }

    fn commit(self) {
        self.parent.append(self.messages);
    }
}

pub trait Instrument {
    const INSTRUMENT_NAME: &'static str;
}

fn construct_instrument(prefix: Option<&str>, i: &str, key: InstrumentKey) -> String {
    let mut instrument = String::new();

    if let Some(p) = prefix {
        instrument.push_str(p);
        instrument.push_str(" / ");
    }

    instrument.push_str(i);

    match key {
        InstrumentKey::Unkeyed => (),
        InstrumentKey::Array(i) => instrument.push_str(&i.to_string()),
        InstrumentKey::Map(k) => instrument.push_str(&k),
    }

    instrument
}
