use axum::{Json, extract::{Path, State}, Extension};
use crate::{PublicPost, ApiError, GetResponse, GetAllPosts, AppState, services::posts_sqlx::DynPostSqlxService};

use crate::services::posts::{DynPostService, get_post_by_id};

pub async fn get_posts(
    axum::extract::Query(params): axum::extract::Query<GetAllPosts>,
    Extension(posts_service): Extension<DynPostService>
) -> Result<Json<GetResponse<Vec<PublicPost>>>, ApiError> {
    let posts: Vec<PublicPost> = posts_service.get_posts(params.page, params.size, params.sort).await?; 

    Ok(Json(GetResponse{
        data: posts,
    }))
}

pub async fn get_post(
    Path(id): Path<i32>,
    Extension(posts_service): Extension<DynPostService>,
) -> Result<Json<GetResponse<PublicPost>>, ApiError> {
    let post: PublicPost = posts_service.get_post_by_id(id).await?;

    Ok(Json(GetResponse{
        data: post,
    }))
}

pub async fn get_post_new(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<GetResponse<PublicPost>>, ApiError> {
    let post: PublicPost = get_post_by_id(&state.database, id).await?; 

    Ok(Json(GetResponse{
        data: post,
    }))
}

pub async fn init() -> String {
    "Hello World".to_string()
}

// pub async fn get_sqlx_post(
//     Path(id): Path<i32>,
//     Extension(posts_service): Extension<DynPostSqlxService>,
// ) -> Result<Json<GetResponse<PublicPost>>, ApiError> {
//     let post: PublicPost = posts_service.get_post_by_id(id).await?;
//
//     Ok(Json(GetResponse{
//         data: post,
//     }))
// }

