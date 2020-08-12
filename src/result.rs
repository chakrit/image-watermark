use std::{convert::From, fmt::Display, fmt::Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // IoError happens when an std::fs operations are used implicitly by a
    // convenience helper/ctors.
    IoError(std::io::Error),

    // LoadError happens when loading of a resource (font, images etc.) fails.
    LoadError(String),

    // ImageError happens when image manipulation fails.
    ImageError(image::ImageError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<image::ImageError> for Error {
    fn from(err: image::ImageError) -> Self {
        Error::ImageError(err)
    }
}
