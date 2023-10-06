use domain::models::Post;
use serde::Serialize;

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    Post(Post),
    Posts(Vec<Post>),
}

#[derive(Serialize)]
pub struct Response {
    pub body: ResponseBody,
}
