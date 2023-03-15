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

    pub fn errornous(&self) -> bool {
        self.errornous
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

#[cfg(test)]
mod test {
    use super::{Compile, Compiler, ErrorStackCompiler, Severity, Validate};

    type TestCompiler = ErrorStackCompiler<String>;

    impl Compile<(&str, usize)> for TestCompiler {
        type Output = usize;

        fn compile(&mut self, (source, rhs): (&str, usize)) -> Result<usize, String> {
            match source.parse::<usize>() {
                Ok(v) if v >= 10000 => {
                    self.warn(format!("huge lhs {v}"));
                    Ok(v + rhs)
                }
                Ok(v) if v >= 100 => {
                    self.info(format!("large lhs {v}"));
                    Ok(v + rhs)
                }
                Ok(v) => Ok(v + rhs),
                Err(e) => {
                    self.error(e.to_string());
                    Ok(0)
                }
            }
        }
    }

    impl Validate<(&str, &str)> for TestCompiler {
        fn validate(&mut self, (lhs, rhs): (&str, &str)) -> Result<bool, String> {
            Ok(lhs >= rhs)
        }
    }

    #[test]
    fn compiler_works() {
        let mut compiler = TestCompiler::new();
        assert_eq!(compiler.compile(("22", 20)), Ok(42));
    }

    #[test]
    fn compiler_info_passes() {
        let mut compiler = TestCompiler::new();
        let compiled = compiler.compile(("100", 28));
        let messages = compiler.messages();
        assert_eq!(compiled, Ok(128));
        assert_eq!(
            messages,
            vec![(Severity::Information, "large lhs 100".into())]
        );
    }

    #[test]
    fn compiler_warn_passes() {
        let mut compiler = TestCompiler::new();
        let compiled = compiler.compile(("16000", 384));
        let messages = compiler.messages();
        assert_eq!(compiled, Ok(16384));
        assert_eq!(messages, vec![(Severity::Warning, "huge lhs 16000".into())]);
    }

    #[test]
    fn compiler_error_stops() {
        let mut compiler = TestCompiler::new();
        let compiled = compiler.compile(("not a number", 384));
        let messages = compiler.messages();
        assert_eq!(compiled, Ok(0));
        assert_eq!(
            messages,
            vec![(Severity::Error, "invalid digit found in string".into())]
        );
    }

    #[test]
    fn compiler_validates() {
        let mut compiler = TestCompiler::new();
        assert_eq!(compiler.ensure(("latter", "former")), Ok(true));
        assert_eq!(compiler.ensure(("Ash", "Sephira")), Ok(false));
    }
}
