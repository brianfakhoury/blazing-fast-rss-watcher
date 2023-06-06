use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ArticleInfo {
    pub title: String,
    pub description: String,
    pub link: String,
}

impl std::fmt::Debug for ArticleInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Article: {}\nDescription: {}\nLink: {}\n",
            self.title, self.description, self.link
        )
    }
}
