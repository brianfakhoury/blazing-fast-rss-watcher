use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct ArticleInfo {
    pub title: String,
    pub description: String,
    pub link: String,
}
