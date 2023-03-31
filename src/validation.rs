use axum::{async_trait, extract::{FromRequest, rejection::JsonRejection}, Json, RequestExt, response::IntoResponse};
use http::{Request, StatusCode};
use validator::{Validate, ValidationErrors};

pub enum ValidatedJsonRejection {
    JsonRejection(JsonRejection),
    ValidationRejection(ValidationErrors)
}

impl IntoResponse for ValidatedJsonRejection {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::JsonRejection(rejection) => rejection.into_response(),
            Self::ValidationRejection(errs) => (StatusCode::UNPROCESSABLE_ENTITY, format!("{}", errs)).into_response()
        }
    }
}

pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for ValidatedJson<T>
where
    B: Send + 'static,
    S: Send + Sync,
    T: Validate + 'static,
    Json<T>: FromRequest<(), B>,
    <Json<T> as FromRequest<(), B>>::Rejection: Into<JsonRejection>
{
    type Rejection = ValidatedJsonRejection;

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Self(data) = match req.extract::<Json<T>, _>().await {
            Ok(Json(data)) => Self(data),
            Err(err) => return Err(ValidatedJsonRejection::JsonRejection(err.into()))
        };
        data.validate().map_err(ValidatedJsonRejection::ValidationRejection)?;
        Ok(Self(data))
    }
}
