use std::fmt::Display;
#[derive(Debug)]
pub struct TryFromValueError {
    pub expected: String,
    pub got: String,
}
impl TryFromValueError {
    pub fn new<T>(expected: &str, got: &str) -> Result<T, Self> {
        Err(Self {
            expected: expected.into(),
            got: got.into(),
        })
    }
}
impl Display for TryFromValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expected {}, found {}", self.expected, self.got)
    }
}
#[derive(Debug)]
pub struct ScannerError {
    message: String,
    pub line: usize,
}
impl ScannerError {
    pub fn new<T>(message: &str, line: usize) -> Result<T, Self> {
        Err(Self {
            message: message.into(),
            line,
        })
    }
}
impl Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug)]
pub struct CompilerError(String, usize);

impl CompilerError {
    pub fn new(message: &str, line: usize) -> Self {
        Self(message.into(), line)
    }
    pub fn set_line(&mut self, line: usize) {
        self.1 = line;
    }
}
impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] Error {}", self.1, self.0)
    }
}
impl From<ScannerError> for CompilerError {
    fn from(e: ScannerError) -> Self {
        Self(format!(":{}", e), e.line)
    }
}

#[derive(Debug)]
pub struct VmError(pub String, pub i32);
impl VmError {
    pub fn new<T>(message: String) -> Result<T, Self> {
        Err(Self(message, 70))
    }
}
impl std::error::Error for VmError {}
impl Display for VmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<ScannerError> for VmError {
    fn from(e: ScannerError) -> Self {
        Self(format!("{}", e), 65)
    }
}
impl From<CompilerError> for VmError {
    fn from(e: CompilerError) -> Self {
        Self(format!("{}", e), 65)
    }
}
impl From<String> for VmError {
    fn from(s: String) -> Self {
        Self(s, 70)
    }
}

impl From<TryFromValueError> for VmError {
    fn from(e: TryFromValueError) -> Self {
        Self(format!("{}", e), 70)
    }
}
