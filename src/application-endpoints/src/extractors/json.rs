use async_trait::async_trait;
use axum::{
    extract::{rejection::JsonRejection, FromRequest, RequestParts},
    http::StatusCode,
    BoxError,
};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

// * Json extractor with custom error handling
pub(crate) struct JsonInput<T>(pub(crate) T);

#[async_trait]
impl<B, T> FromRequest<B> for JsonInput<T>
where
    T: DeserializeOwned,
    B: axum::body::HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = (StatusCode, axum::Json<Value>);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let error_msg = "Error on parsing json";
        match axum::Json::<T>::from_request(req).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                match rejection {
                    JsonRejection::JsonDataError(err) => {
                        tracing::warn!("{error_msg}: {:#?}", err.to_string());
                    }
                    JsonRejection::JsonSyntaxError(err) => {
                        tracing::warn!("{error_msg}: {:#?}", err.to_string());
                    }
                    JsonRejection::MissingJsonContentType(err) => {
                        tracing::warn!("{error_msg}: {:#?}", err.to_string());
                    }
                    JsonRejection::BytesRejection(err) => {
                        tracing::warn!("{error_msg}: {:#?}", err.to_string());
                    }
                    err => {
                        tracing::warn!("{error_msg}: {:#?}", err.to_string());
                    }
                };

                Err((
                    StatusCode::BAD_REQUEST,
                    axum::Json(json!({ "error": "Error on parsing json" })),
                ))
            }
        }
    }
}
