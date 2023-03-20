use std::task::{Context, Poll};

use anyhow::Result;
use axum::{
    http::Request, body::{Body, BoxBody}, response::Response, extract::FromRequestParts, async_trait
};
use axum_extra::extract::CookieJar;
use futures::future::BoxFuture;
use http::{StatusCode, request::Parts};
use rand::RngCore;
use tower::{Layer, Service};

use crate::{
    service::sessions::{SessionService, SessionVerifyError}, constants, domain::users::User,
};

#[derive(Clone)]
pub struct AuthLayer<T: SessionService + Clone> {
    service: T
}

impl<T: SessionService + Clone> AuthLayer<T> {
    pub fn new(session_service: T) -> Self {
        Self { service: session_service }
    }
}

impl<S, T: SessionService + Clone> Layer<S> for AuthLayer<T> {
    type Service = AuthMiddleware<S, T>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware {
            inner,
            session_service: self.service.clone()
        }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S, T: SessionService> {
    inner: S,
    session_service: T
}

impl<S, T> Service<Request<Body>> for AuthMiddleware<S, T>
where
    S: Service<Request<Body>, Response = Response> + Send + Clone + 'static,
    S::Future: Send + 'static,
    T: SessionService + Send + Sync + Clone + 'static
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request<Body>) -> Self::Future {
        let session_cookie = match CookieJar::from_headers(&request.headers()).get(constants::SESSION_ID) {
            Some(cookie) => cookie.clone(),
            None => {
                let response = Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(BoxBody::default())
                    .unwrap();
                return Box::pin(async move { Ok(response) });
            }
        };
        
        let session_service = self.session_service.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            let user = match session_service.verify(session_cookie.value()).await {
                Ok(user) => user,
                Err(SessionVerifyError::Missing) => { 
                    let response = Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body(BoxBody::default())
                        .unwrap();
                    return Ok(response)
                },
                Err(SessionVerifyError::Unknown) => {
                    let response = Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(BoxBody::default())
                        .unwrap();
                    return Ok(response)
                }
            };
            
            request.extensions_mut().insert(user);
            Ok(inner.call(request).await?)
        })
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<User>()
            .cloned()
            .ok_or((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Can't extract User. Is `AuthLayer` enabled?",
            ))
    }
}

pub fn generate_session_id() -> String {
    let mut nums: [u64; 4] = [0, 0, 0, 0];
    let mut rng = rand::thread_rng();
    nums[0] = rng.next_u64();
    nums[1] = rng.next_u64();
    nums[2] = rng.next_u64();
    nums[3] = rng.next_u64();
    format!("{}{}{}{}", nums[0], nums[1], nums[2], nums[3])
}
