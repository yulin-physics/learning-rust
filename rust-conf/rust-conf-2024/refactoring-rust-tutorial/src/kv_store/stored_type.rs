// talk about app domain instead of impl details

use axum::{body::Bytes, headers::ContentType, response::IntoResponse};
use image::DynamicImage;

use super::{image_response::ImageResponse, kv_error::KVError};

pub enum StoredType {
    Image(DynamicImage),
    Other((String, Bytes)),
}

impl StoredType {
    pub fn new(content_type: ContentType, bytes: Bytes) -> Result<Self, KVError> {
        // if content_type.to_string().starts_with("image/") {
        // }
        todo!()
    }
}

impl IntoResponse for &StoredType {
    fn into_response(self) -> axum::response::Response {
        match self {
            StoredType::Image(image) => ImageResponse::try_from(image).into_response(),
            StoredType::Other((content_type, data)) => {
                ([("content-type", content_type)], data.clone()).into_response()
            }
        }
    }
}
