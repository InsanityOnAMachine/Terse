use serde::{ Serialize, Deserialize };

// This could all be in network, in posts or publishing, etc...

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub title: String,
    pub content: String,
}