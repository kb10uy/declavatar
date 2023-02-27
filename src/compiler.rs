use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Severity {
    Information,
    Warning,
    Error,
}

pub trait Compiler {
    type Error;

    fn info(&mut self, message: String);
    fn warn(&mut self, message: String);
    fn error(&mut self, message: String);

    fn parse<T>(&mut self, source: T) -> Result<<Self as Compile<T>>::Output, Self::Error>
    where
        Self: Compile<T>;

    fn ensure<T>(&mut self, source: T) -> Result<bool, Self::Error>
    where
        Self: Validate<T>;
}

pub trait Compile<T>
where
    Self: Compiler,
{
    type Output;

    fn compile(&mut self, source: T) -> Result<Self::Output, Self::Error>;
}

pub trait Validate<T>
where
    Self: Compiler,
{
    fn validate(&mut self, source: T) -> Result<bool, Self::Error>;
}

#[derive(Debug)]
pub struct ErrorStackCompiler<E> {
    messages: Vec<(Severity, String)>,
    errornous: bool,
    _error_type: PhantomData<fn() -> E>,
}

impl<E> ErrorStackCompiler<E> {
    pub fn new() -> ErrorStackCompiler<E> {
        ErrorStackCompiler {
            messages: vec![],
            errornous: false,
            _error_type: Default::default(),
        }
    }

    pub fn messages(self) -> Vec<(Severity, String)> {
        self.messages
    }
}

impl<E> Compiler for ErrorStackCompiler<E> {
    type Error = E;

    fn info(&mut self, message: String) {
        self.messages.push((Severity::Information, message));
    }

    fn warn(&mut self, message: String) {
        self.messages.push((Severity::Warning, message));
    }

    fn error(&mut self, message: String) {
        self.errornous = true;
        self.messages.push((Severity::Error, message));
    }

    fn parse<T>(&mut self, source: T) -> Result<<Self as Compile<T>>::Output, Self::Error>
    where
        Self: Compile<T>,
    {
        <Self as Compile<T>>::compile(self, source)
    }

    fn ensure<T>(&mut self, source: T) -> Result<bool, Self::Error>
    where
        Self: Validate<T>,
    {
        <Self as Validate<T>>::validate(self, source)
    }
}
