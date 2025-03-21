use async_trait::async_trait;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::error::Error;
use std::vec::IntoIter;
use uuid::Uuid;
pub mod get;
pub mod new;
mod sqlite;

// TODO: publish on on crates.io
// TODO: implement traits on the "connection" so that we can interface with sqlite, postgrees, and mysql.

#[derive(Debug)]
pub struct Comment {
    pub id: Uuid,
    pub ip: String,
    pub username: String,
    pub comment: String,
    pub timestamp: String,
    pub visible: i64,
    pub post_url: String,
}

impl Serialize for Comment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Comment", 7)?;

        state.serialize_field("id", &self.id.to_string())?; // Convert UUID to string
        state.serialize_field("ip", &self.ip)?;
        state.serialize_field("username", &self.username)?;
        state.serialize_field("comment", &self.comment)?;
        state.serialize_field("timestamp", &self.timestamp)?;
        state.serialize_field("visible", &self.visible)?;
        state.serialize_field("post_url", &self.post_url)?;

        // Finalize the serialization
        state.end()
    }
}

impl IntoIterator for Comment {
    type Item = String;
    type IntoIter = IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            self.id.to_string(),
            self.ip,
            self.username,
            self.comment,
            self.timestamp,
            self.visible.to_string(),
            self.post_url,
        ]
        .into_iter()
    }
}

// TODO: where does this get used?
#[derive(Debug)]
#[allow(dead_code)]
pub struct CommentError {
    error: String,
    comment: Comment,
}

// TODO: where does this get used?
pub enum CommentResult<Comment, CommentError> {
    Ok(Comment),
    Err(CommentError),
}

#[async_trait]
pub trait Database {
    fn connect(connection_str: &str) -> Result<Self, Box<dyn Error>>
    where
        // This is added to the connection method to make sure that *Self* is a concrete type, which is required for returning *Self* in the result. Essentially, we do not know the size of Self to place on the stack at runtime.
        Self: Sized;

    fn initialize_database(&self) -> Result<(), Box<dyn Error>>;

    // Simple CRUD
    fn create_comment(&self, comment: Comment) -> Result<(), Box<dyn Error>>;
    fn read_comment(&self, id: Uuid) -> Result<(), Box<dyn Error>>;
    fn read_comments(&self, comment: Comment) -> Result<(), Box<dyn Error>>;
    fn update_comment(&self, comment: Comment) -> Result<(), Box<dyn Error>>;
    fn delete_comment(&self, comment: Comment) -> Result<(), Box<dyn Error>>;
    fn validate_comment(&self, comment: Comment) -> Result<(), Box<dyn Error>>;
}
