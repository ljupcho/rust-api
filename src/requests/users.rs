use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct UserRegisterRequest {
    #[validate(required, length(min = 1))]
    pub username: Option<String>,
    #[validate(required, length(min = 1), email(message = "email is invalid"))]
    pub email: Option<String>,
    #[validate(required, length(min = 1))]
    pub password: Option<String>,
}


#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct UserLoginRequest {
    #[validate(required, length(min = 1), email(message = "email is invalid"))]
    pub email: Option<String>,
    #[validate(required, length(min = 1))]
    pub password: Option<String>,
}
