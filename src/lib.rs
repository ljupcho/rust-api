pub mod handlers;
pub mod shared;
pub mod services;
pub mod stores;
pub mod requests;
mod mappers;

use std::sync::Arc;
use sea_orm::DatabaseConnection;
use axum::{
    response::{IntoResponse, Response},
    Json, Router,
    routing::get, Extension,
};
use hyper::StatusCode;
use services::posts::{PostService, DynPostService};
use stores::posts::PostStore;
use utoipa::{IntoParams, ToSchema};
use core::fmt;
use serde::{de, Serialize, Deserialize, Deserializer};
use std::str::FromStr;

#[derive(Clone)]
pub struct AppState {
    pub database: DatabaseConnection,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse<T> {
    #[schema(example = "Error response data")]
    pub error: T,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct GetResponse<T> {
    #[schema(example = "Response data")]
    pub data: T,
}

#[derive(Debug)]
pub enum ApiError {
    RecordNotFound,
    ParamError(String),
    DatabaseError(sea_orm::DbErr),
    FileNotFound(String),
    IoError(std::io::Error),
    UnknownError(String),
}

// Convert sea_orm::DbErr into our custom ApiError allows ? 
// to be called on sea_orm querys such as find_by_id().await? etc. Pushing up the error to the caller.
// Which most of the time is a web handler. 
// Which with impl IntoResponse for ApiError can convert these errors into errors
// with response codes and good messages
impl From<sea_orm::DbErr> for ApiError {
    fn from(error: sea_orm::DbErr) -> Self {
        ApiError::DatabaseError(error)
    }
}
impl From<std::io::Error> for ApiError {
    fn from(error: std::io::Error) -> Self {
        ApiError::IoError(error)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status_code, error_message) = match self {
            ApiError::DatabaseError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("An unexpected exception has occured: {}", err),
            ),
            ApiError::RecordNotFound => (StatusCode::NOT_FOUND, r#"Record not found"#.to_string()),
            ApiError::FileNotFound(err) => {
                (StatusCode::NOT_FOUND, format!("File not found: {}", err))
            }
            ApiError::IoError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("IO Error: {}", err),
            ),
            ApiError::UnknownError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unknown error: {}", err),
            ),
            ApiError::ParamError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Missing: {}", err),
            ),
        };

        (
            status_code,
            Json(ErrorResponse {
                error: error_message,
            }),
        ).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicPost {
        pub id: i32,
        pub name: String,
}

#[derive(Deserialize, Clone, IntoParams, ToSchema)]
pub struct GetAllPosts {
        #[serde(default, deserialize_with = "empty_string_as_none")]
        #[schema(example = "sort = name | latest")]
        pub sort: Option<String>,
        #[serde(default, deserialize_with = "empty_string_as_none")]
        pub size: Option<u64>,
        #[serde(default, deserialize_with = "empty_string_as_none")]
        pub page: Option<u64>,
}

fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

pub struct AppRouter; 

impl AppRouter {
    pub fn build(db: DatabaseConnection) -> Router {
        Router::new()
            .nest("/api", PostRouter::new_router(db))
    }
}

pub struct PostRouter;

impl PostRouter {
    pub fn new_router(db: DatabaseConnection) -> Router {
        let store = Arc::new(PostStore::new(db));
        let srv = Arc::new(PostService::new(store)) as DynPostService;

        Router::new()
            .route("/posts", get(handlers::posts::get_posts))
            .route("/posts/:id", get(handlers::posts::get_post))
            // .with_state(srv)
            .layer(Extension(srv))
    }
}


