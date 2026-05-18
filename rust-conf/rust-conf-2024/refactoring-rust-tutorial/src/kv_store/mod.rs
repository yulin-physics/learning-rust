use std::io::Cursor;

use axum::{
    extract::{Path, State},
    headers::ContentType,
    response::IntoResponse,
    TypedHeader,
};
use hyper::{body::Bytes, StatusCode};
use image::ImageOutputFormat;

use crate::SharedState;

use self::{image_response::ImageResponse, kv_error::KVError};

mod image_response;
mod kv_error;
mod stored_type;

pub async fn post_kv(
    Path(key): Path<String>,
    TypedHeader(content_type): TypedHeader<ContentType>,
    State(state): State<SharedState>,
    data: Bytes,
) -> Result<String, ()> {
    state
        .write()
        .unwrap()
        .db
        .insert(key, (content_type.to_string(), data));
    Ok("OK".to_string())
}

pub async fn get_kv(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, KVError> {
    match state.read()?.db.get(&key) {
        Some((content_type, data)) => Ok(([("content-type", content_type.clone())], data.clone())),
        None => Err(KVError::not_found()),
    }
}

pub async fn grayscale(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, KVError> {
    let image = match state.read().unwrap().db.get(&key) {
        Some((content_type, data)) => {
            if content_type == "image/png" {
                image::load_from_memory(&data).unwrap()
            } else {
                return Err(KVError::forbidden());
            }
        }
        None => return Err(KVError::not_found()),
    };
    ImageResponse::try_from(&image.grayscale())
}
