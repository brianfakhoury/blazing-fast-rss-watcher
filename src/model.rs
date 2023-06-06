use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct MyItem {
    pub title: String,
    pub link: String,
}
