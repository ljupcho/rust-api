use async_trait::async_trait;
use std::sync::Arc;
use sea_orm::EntityTrait;
use crate::ApiError;
use sea_orm::DatabaseConnection;
use sea_orm::{QueryOrder, PaginatorTrait};

pub type DynPostStore = Arc<dyn PostStoreTrait + Send + Sync>;

#[async_trait]
pub trait PostStoreTrait {
    async fn get_posts(&self, page: Option<u64>, size: Option<u64>, sort: Option<String>) -> Result<Vec<entity::post::Model>, ApiError>;
    async fn get_post_by_id(&self, id: i32) -> Result<entity::post::Model, ApiError>;
}

#[derive(Clone)]
pub struct PostStore {
    db: DatabaseConnection,
}

impl PostStore {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl PostStoreTrait for PostStore {
    async fn get_posts(&self, page: Option<u64>, size: Option<u64>, sort: Option<String>) -> Result<Vec<entity::post::Model>, ApiError> {
        // Get column to be used for order by.
        // This supports ordering by name or created_at columns.
        let order = match sort.unwrap_or_default().as_str() {
                "name" => entity::post::Column::Name,
                "latest" => entity::post::Column::CreatedAt,
                _ => entity::post::Column::Id,
            };

        let posts = entity::post::Entity::find()
                    .order_by_desc(order)
                    .paginate(&self.db, size.unwrap_or(u64::MAX));

        let data = posts.fetch_page(page.unwrap_or(0))
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query for fetching paginated posts: {:?}", e);
                e
            })?;

        if !data.is_empty() {
            Ok(data)
        } else {
            Err(ApiError::RecordNotFound)
        } 
    }

    async fn get_post_by_id(&self, id: i32) -> Result<entity::post::Model, ApiError> {
        match entity::post::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query for fetching post by id: {:?}", e);
                e
            })?
            {
                Some(post) => Ok(post),
                None => Err(ApiError::RecordNotFound),
            }
    }
}
