// polymorphic error type for convienence
pub type Error = Box<dyn std::error::Error>;

// shorthand macro to create an OpenFocusError including line number, file name,
// and error type
#[macro_export]
macro_rules! err {
    ($kind:ident) => {
        Box::new(OpenFocusError {
            file: file!(),
            line: line!(),
            kind: OpenFocusErrorType::$kind,
        })
    }
}

#[derive(Debug)]
pub struct OpenFocusError {
    pub file: &'static str,
    pub line: u32,
    pub kind: OpenFocusErrorType,
}

impl std::error::Error for OpenFocusError {}
impl std::fmt::Display for OpenFocusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}:{}", self.kind, self.file, self.line)
    }
}

// error type specific to the project
#[derive(Debug)]
pub enum OpenFocusErrorType {
    Parse,
    NotFound,
    InvalidArgument,
    Unknown,
}

impl std::error::Error for OpenFocusErrorType {}
impl std::fmt::Display for OpenFocusErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            OpenFocusErrorType::Parse => "Parse Error",
            OpenFocusErrorType::NotFound => "Item Not Found Error",
            OpenFocusErrorType::InvalidArgument => "Invalid Argument",
            OpenFocusErrorType::Unknown => "Unknown Error",
        };
        write!(f, "{}", s)
    }
}
