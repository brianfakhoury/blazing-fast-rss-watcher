use serde_derive::Serialize;

#[derive(Serialize, Debug)]
pub struct Article {
    pub title: String,
    pub description: Option<String>,
    pub link: String,
}
