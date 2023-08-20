use anyhow::Result;
use sea_orm::{DatabaseConnection, EntityTrait};
use std::sync::Arc;
use async_trait::async_trait;
use crate::mappers::posts::get_public_post;
use crate::{ApiError, PublicPost};
use crate::stores::posts::DynPostStore;

pub type DynPostService = Arc<dyn PostServiceTrait + Send + Sync>;

#[async_trait]
pub trait PostServiceTrait {
    async fn get_posts(&self, page: Option<u64>, size: Option<u64>, sort: Option<String>) -> Result<Vec<PublicPost>, ApiError>;
    async fn get_post_by_id(&self, id: i32) -> Result<PublicPost, ApiError>;
}

#[derive(Clone)]
pub struct PostService {
    store: DynPostStore,
}

impl PostService {
    pub fn new(store: DynPostStore) -> Self {
        Self { store }
    }
}
// #[derive(Clone)]
// pub struct PostService<'a> {
//     store: &'a PostStore<'a>,
// }
//
// impl<'a> PostService<'a> {
//     pub fn new(store: &'a PostStore<'a>) -> Self {
//         Self { store }
//     }
// }

#[async_trait]
impl PostServiceTrait for PostService {
    async fn get_posts(&self, page: Option<u64>, size: Option<u64>, sort: Option<String>) -> Result<Vec<PublicPost>, ApiError> {
        let posts = self.store.get_posts(page, size, sort)
            .await
            .map_err(|e| {
                tracing::error!("Failed to get paginated posts: \"{:?}\"", e);
                e
            })?;

        let public_posts: Vec<PublicPost> = posts.into_iter()
            .map(|post| get_public_post(post))
            .collect();

        Ok(public_posts)
    }

    async fn get_post_by_id(&self, id: i32) -> Result<PublicPost, ApiError> {
        let res = self.store.get_post_by_id(id)
            .await
            .map_err(|e| {
                tracing::error!("Failed to get post: \"{:?}\" for {:}", e, id);
                e
            })?;

        Ok(
            PublicPost{
                id: res.id,
                name: res.name,
            }
        )
    }
}

pub async fn get_post_by_id(db: &DatabaseConnection, id: i32) -> Result<PublicPost, ApiError> {
        let res = entity::post::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query for fetching post by id: {:?}", e);
                e
            })?.unwrap();

        Ok(
            PublicPost{
                id: res.id,
                name: res.name,
            }
        )
    }


