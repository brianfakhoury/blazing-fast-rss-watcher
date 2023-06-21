use serde_derive::Serialize;

#[derive(Serialize)]
pub struct Article {
    pub title: String,
    pub description: Option<String>,
    pub link: String,
}

impl std::fmt::Debug for Article {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Article: {}\nDescription: {}\nLink: {}",
            self.title,
            self.description.as_ref().unwrap_or(&String::from("")),
            self.link
        )
    }
}
