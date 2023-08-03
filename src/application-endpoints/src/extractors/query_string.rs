use async_trait::async_trait;
use axum::{
    extract::{rejection::QueryRejection, FromRequest, RequestParts},
    http::StatusCode,
};
use serde::de::DeserializeOwned;

// * Path extractor with custom error handling
pub(crate) struct QueryInput<T>(pub(crate) T);

#[async_trait]
impl<B, T> FromRequest<B> for QueryInput<T>
where
    T: DeserializeOwned + Send,
    B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        match axum::extract::Query::<T>::from_request(req).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                match rejection {
                    QueryRejection::FailedToDeserializeQueryString(inner) => {
                        tracing::warn!("QueryInput error: {inner:#?}");
                    }

                    _ => {
                        tracing::warn!("QueryInput error: Unhandled query_string rejection");
                    }
                };

                Err((StatusCode::BAD_REQUEST, "Bad Request."))
            }
        }
    }
}
