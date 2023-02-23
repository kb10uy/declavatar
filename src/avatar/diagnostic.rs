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

pub trait Diagnostic {
    fn info<I: Instrument>(&mut self, instrument: &I, message: String);
    fn warn<I: Instrument>(&mut self, instrument: &I, message: String);
    fn error<I: Instrument>(&mut self, instrument: &I, message: String);
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

    pub fn of<'a, I: Instrument>(&'a mut self, parent: &I) -> ScopedCompilerInfo<'a, CompilerInfo> {
        let prefix = format!("{} / ", parent.show());
        ScopedCompilerInfo {
            parent: self,
            prefix,
            messages: vec![],
            errornous: false,
        }
    }
}

impl Diagnostic for CompilerInfo {
    fn info<I: Instrument>(&mut self, instrument: &I, message: String) {
        self.messages
            .push(Message::info(instrument.show(), message));
    }

    fn warn<I: Instrument>(&mut self, instrument: &I, message: String) {
        self.messages
            .push(Message::warn(instrument.show(), message));
    }

    fn error<I: Instrument>(&mut self, instrument: &I, message: String) {
        self.messages
            .push(Message::error(instrument.show(), message));
        self.errornous = true;
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

impl<'p, P: Diagnostic> ScopedCompilerInfo<'p, P> {
    pub fn errornous(&self) -> bool {
        self.errornous
    }

    pub fn of<'a: 'p, I: Instrument>(
        &'a mut self,
        parent: &I,
    ) -> ScopedCompilerInfo<'a, ScopedCompilerInfo<'a, P>> {
        let prefix = format!("{}{} / ", self.prefix, parent.show());
        ScopedCompilerInfo {
            parent: self,
            prefix,
            messages: vec![],
            errornous: false,
        }
    }
}

impl<'p, P: Diagnostic> Diagnostic for ScopedCompilerInfo<'p, P> {
    fn info<I: Instrument>(&mut self, instrument: &I, message: String) {
        self.messages.push(Message::info(
            format!("{}{}", self.prefix, instrument.show()),
            message,
        ));
    }

    fn warn<I: Instrument>(&mut self, instrument: &I, message: String) {
        self.messages.push(Message::warn(
            format!("{}{}", self.prefix, instrument.show()),
            message,
        ));
    }

    fn error<I: Instrument>(&mut self, instrument: &I, message: String) {
        self.messages.push(Message::error(
            format!("{}{}", self.prefix, instrument.show()),
            message,
        ));
        self.errornous = true;
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
    fn show(&self) -> String;
}

impl<I: Instrument> Instrument for &I {
    fn show(&self) -> String {
        let this = *self;
        this.show()
    }
}

impl<I: Instrument> Instrument for (I, usize) {
    fn show(&self) -> String {
        let (instrument, index) = self;
        format!("{} #{index}", instrument.show())
    }
}

impl<'a, I: Instrument> Instrument for (I, String) {
    fn show(&self) -> String {
        let (instrument, key) = self;
        format!("{} #{key}", instrument.show())
    }
}

impl<'a, I: Instrument> Instrument for (I, &'a str) {
    fn show(&self) -> String {
        let (instrument, key) = self;
        format!("{} #{key}", instrument.show())
    }
}
