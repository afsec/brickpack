use async_trait::async_trait;
use axum::{
    extract::{path::ErrorKind, rejection::PathRejection, FromRequest, RequestParts},
    http::StatusCode,
};
use serde::de::DeserializeOwned;

// * Path extractor with custom error handling
pub(crate) struct PathInput<T>(pub(crate) T);

#[async_trait]
impl<B, T> FromRequest<B> for PathInput<T>
where
    T: DeserializeOwned + Send,
    B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        match axum::extract::Path::<T>::from_request(req).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                match rejection {
                    PathRejection::FailedToDeserializePathParams(inner) => {
                        let kind = inner.into_kind();
                        match &kind {
                            ErrorKind::WrongNumberOfParameters { .. } => {
                                tracing::error!("Error on parsing path: WrongNumberOfParameters");
                            }

                            ErrorKind::ParseErrorAtKey { .. } => {
                                tracing::error!("Error on parsing path: ParseErrorAtKey");
                            }

                            ErrorKind::ParseErrorAtIndex { .. } => {
                                tracing::error!("Error on parsing path: ParseErrorAtIndex");
                            }

                            ErrorKind::ParseError { .. } => {
                                tracing::error!("Error on parsing path: ParseError");
                            }

                            ErrorKind::InvalidUtf8InPathParam { .. } => {
                                tracing::error!("Error on parsing path: InvalidUtf8InPathParam");
                            }

                            ErrorKind::UnsupportedType { .. } => {
                                // this error is caused by the programmer using an unsupported type
                                // (such as nested maps) so respond with `500` instead
                                tracing::error!("Error on parsing path: UnsupportedType");
                            }

                            ErrorKind::Message(_) => {
                                tracing::error!("Error on parsing path: Message");
                            }

                            _ => {
                                tracing::error!(
                                    "Error on parsing path: Unhandled deserialization error."
                                );
                            }
                        };
                    }
                    PathRejection::MissingPathParams(error) => {
                        tracing::error!("Error on parsing path: {:#?}", error.to_string());
                    }
                    _ => {
                        tracing::error!("Error on parsing path: Unhandled path rejection");
                    }
                };

                Err((StatusCode::BAD_REQUEST, "Bad Request"))
            }
        }
    }
}
