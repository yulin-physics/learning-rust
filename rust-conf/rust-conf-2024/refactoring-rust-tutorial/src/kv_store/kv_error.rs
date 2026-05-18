use std::sync::PoisonError;

use axum::response::IntoResponse;
use hyper::StatusCode;
use image::ImageError;

/// create our own semantics
#[derive(Debug)]
pub struct KVError(StatusCode, String);

impl KVError {
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        // to_(string) operations have a good chance of being expensive, allocates new memory/buffer
        // into and as are cheap
        // read name conventions in rust
        KVError(status, message.into())
    }

    pub(crate) fn not_found() -> KVError {
        KVError::new(StatusCode::NOT_FOUND, "Key not found")
    }

    pub(crate) fn forbidden() -> KVError {
        KVError::new(
            StatusCode::FORBIDDEN,
            "Not possible to grayscale this type of image",
        )
    }
}

impl IntoResponse for KVError {
    fn into_response(self) -> axum::response::Response {
        (self.0, self.1).into_response()
    }
}

//
impl std::fmt::Display for KVError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KVError: {}", self.1)
    }
}

// to propagate the error using ?
// app is in unrecoverable state
impl<T> From<PoisonError<T>> for KVError {
    fn from(value: PoisonError<T>) -> Self {
        todo!()
    }
}

impl From<ImageError> for KVError {
    fn from(value: ImageError) -> Self {
        todo!()
    }
}

impl std::error::Error for KVError {}
