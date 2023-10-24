use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

pub type DynPostSqlxStore = Arc<dyn PostSqlxStoreTrait + Send + Sync>;

#[async_trait]
pub trait PostSqlxStoreTrait {
    // async fn get_post_by_id(&self, id: i32) -> Result<PostModel, ApiError>;
}

#[derive(Clone)]
pub struct PostSqlxStore {
    db: Pool<Postgres>,
}

impl PostSqlxStore {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl PostSqlxStoreTrait for PostSqlxStore {
    // async fn get_post_by_id(&self, id: i32) -> Result<PostModel, ApiError> {
    //     let res = query_as!(
    //         PostModel,
    //         r#"
    //         select id, title, name, text 
    //         from post 
    //         where id = $1
    //             "#,
    //             id,
    //         )
    //         .fetch_one(&self.db)
    //         .await;
    //
    //     match res {
    //         Ok(row) => Ok(row),
    //         Err(err) => {
    //             match err {
    //                 sqlx::error::Error::RowNotFound => {
    //                     return Err(ApiError::RecordNotFound)
    //                 }
    //                 _ => {
    //                     eprintln!("An unknown error occurred: {:?}", err);
    //                 }                
    //             }
    //             Err(ApiError::DatabaseSqlxError(err))
    //         }
    //     }
    // }
}
