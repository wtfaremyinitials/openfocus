pub type Error = Box<std::error::Error>;

#[derive(Debug)]
pub enum OpenFocusError { Parse }

impl std::error::Error for OpenFocusError {}

impl std::fmt::Display for OpenFocusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
