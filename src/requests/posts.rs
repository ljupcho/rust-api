use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct GetPostApiRequest {
    pub name: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
