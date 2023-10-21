use anyhow::Result;
use std::sync::Arc;
use async_trait::async_trait;
use crate::mappers::posts::get_public_sqlx_post;
use crate::stores::posts_sqlx::DynPostSqlxStore;
use crate::{ApiError, PublicPost};

pub type DynPostSqlxService = Arc<dyn PostSqlxServiceTrait + Send + Sync>;

#[async_trait]
pub trait PostSqlxServiceTrait {
    async fn get_post_by_id(&self, id: i32) -> Result<PublicPost, ApiError>;
}

#[derive(Clone)]
pub struct PostSqlxService {
    store: DynPostSqlxStore,
}

impl PostSqlxService {
    pub fn new(store: DynPostSqlxStore) -> Self {
        Self { store }
    }
}

#[async_trait]
impl PostSqlxServiceTrait for PostSqlxService {
    async fn get_post_by_id(&self, id: i32) -> Result<PublicPost, ApiError> {
        let res = self.store.get_post_by_id(id)
            .await
            .map_err(|e| {
                tracing::error!("Failed to get post: \"{:?}\" for {:}", e, id);
                e
            })?;

        Ok(get_public_sqlx_post(res))
    }
}


