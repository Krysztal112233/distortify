use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("{0}")]
    Io(#[from] io::Error),

    #[error("{0}")]
    Image(#[from] image::error::ImageError),
}

#[allow(unused)]
pub(crate) type Result<T> = ::std::result::Result<T, Error>;
