

/// Models the possible errors returned when reading a .class file
#[derive(Debug, PartialEq, Eq)]
pub enum JavaAnalyzeError {
    // Errors related to reading from a Buffer
    UnexpectedEOF,
    InvalidCesu8String,
    
    /// Generic error meaning that the class file is invalid
    InvalidClassData(String),
    UnsupportedVersion(u16, u16),
    InvalidTypeDescriptor(String),
}


pub type Result<T> = std::result::Result<T, JavaAnalyzeError>;

