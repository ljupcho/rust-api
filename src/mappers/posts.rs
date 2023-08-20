use crate::PublicPost;

pub fn get_public_post(post: entity::post::Model) -> PublicPost {
    PublicPost { id: post.id, name: post.name }
}
