//! Error type for wheelhoss
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    FailedToOpenFile(String, Option<std::io::Error>),
    FailedToSplitFilename(String),
    FilesListWriteIncomplete(String),
    IoError(std::io::Error),
    UnableToProcessNonUtf8Path(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match &*self {
            Self::FailedToOpenFile(filename, maybe_error) => {
                let error = match maybe_error {
                    Some(error) => format!(": {}", error),
                    None => "".to_string(),
                };
                write!(f, "Failed to open file \"{}\"{}", filename, error)
            }
            Self::FailedToSplitFilename(filename) => {
                write!(f, "Failed to split filename on \".cpython-\": {}", filename)
            }
            Self::FilesListWriteIncomplete(filename) => {
                write!(f, "Write to files list was incomplete: {}", filename)
            }
            Self::IoError(err) => write!(f, "IoError: {}", err),
            Self::UnableToProcessNonUtf8Path(path) => {
                write!(f, "Unable to process this non-UTF-8 path: {}", path)
            }
        }?;
        Ok(())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}
