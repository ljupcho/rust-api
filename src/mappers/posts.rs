use crate::{PublicPost, PostModel};

pub fn get_public_post(post: entity::post::Model) -> PublicPost {
    PublicPost { id: post.id, name: post.name }
}

pub fn get_public_sqlx_post(post: PostModel) -> PublicPost {
    PublicPost { id: post.id, name: post.name }
}
