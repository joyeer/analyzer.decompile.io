use std::fmt;

#[derive(Debug)]
pub enum AndroidAnalyzeError {
    IoError(std::io::Error),
    InvalidApkFile(String),
    InvalidDexFile(String),
    InvalidManifest(String),
    InvalidResource(String),
    ParseError(String),
}

impl fmt::Display for AndroidAnalyzeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AndroidAnalyzeError::IoError(err) => write!(f, "IO error: {}", err),
            AndroidAnalyzeError::InvalidApkFile(msg) => write!(f, "Invalid APK file: {}", msg),
            AndroidAnalyzeError::InvalidDexFile(msg) => write!(f, "Invalid DEX file: {}", msg),
            AndroidAnalyzeError::InvalidManifest(msg) => write!(f, "Invalid manifest: {}", msg),
            AndroidAnalyzeError::InvalidResource(msg) => write!(f, "Invalid resource: {}", msg),
            AndroidAnalyzeError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for AndroidAnalyzeError {}

impl From<std::io::Error> for AndroidAnalyzeError {
    fn from(err: std::io::Error) -> Self {
        AndroidAnalyzeError::IoError(err)
    }
}

impl From<zip::result::ZipError> for AndroidAnalyzeError {
    fn from(err: zip::result::ZipError) -> Self {
        AndroidAnalyzeError::InvalidApkFile(format!("ZIP error: {}", err))
    }
}

impl From<String> for AndroidAnalyzeError {
    fn from(err: String) -> Self {
        AndroidAnalyzeError::ParseError(err)
    }
}

pub type Result<T> = std::result::Result<T, AndroidAnalyzeError>;
