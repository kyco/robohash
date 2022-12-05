use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("int parsing error")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("error processing image")]
    ImageProcessingError(#[from] image::ImageError),
    #[error("unknown error")]
    Unknown,
}
