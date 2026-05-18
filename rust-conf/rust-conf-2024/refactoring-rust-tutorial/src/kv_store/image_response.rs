use std::io::Cursor;

use axum::{body::Bytes, response::IntoResponse};
use image::{DynamicImage, ImageOutputFormat};

use super::kv_error::KVError;
// zero cost abstraction, everything is dealt with at compile time

// composition, constructing custom type
pub struct ImageResponse(Bytes);

impl ImageResponse {
    pub fn new(bytes: impl Into<Bytes>) -> Self {
        ImageResponse(bytes.into())
    }
}

impl IntoResponse for ImageResponse {
    fn into_response(self) -> axum::response::Response {
        ([("content-type", "image/png")], self.0).into_response()
    }
}

impl TryFrom<&DynamicImage> for ImageResponse {
    type Error = KVError;

    fn try_from(value: &DynamicImage) -> Result<Self, Self::Error> {
        let mut vec: Vec<u8> = Vec::new();

        let mut cursor = Cursor::new(&mut vec);
        value
            .write_to(&mut cursor, ImageOutputFormat::Png)
            .unwrap();
        let bytes: Bytes = vec.into();

        Ok(ImageResponse::new(bytes))
    }
}
