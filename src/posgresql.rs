use std::error::Error;

use async_trait::async_trait;
use sqlite::Connection;
use uuid::Uuid;

use crate::{Comment, Database};

pub struct PostgresSQL {
    pub connection_string: String,
    pub conn: Connection,
}

#[async_trait]
impl Database for PostgresSQL {
    fn connect(connection_str: &str) -> Result<Self, Box<dyn Error>> {
        // 1. Initialize database.
        // 2. Return databse connection.
        todo!();
    }

    fn initialize_database(&self) -> Result<(), Box<dyn Error>> {
        // 1. Check if database file exists.
        // 2. Check if table exists in database.     
        // 3. Return status of initialization.   
        todo!();
    }

    // Simple CRUD
    fn create_comment(&self, comment: Comment) -> Result<(), Box<dyn Error>> {
        // 1. Validate comment contents.
        // 2. Submit comment to database.
        // 3. Return status of comment creation
        todo!();
    }
    
    fn read_comment(&self, id: Uuid) -> Result<(), Box<dyn Error>> {
        // 1. Validate ID.
        // 2. Connect to database.
        // 3. Query for specific comment.
        // 4. Return comment.
        todo!();
    }
    
    fn read_comments(&self, comment: Comment) -> Result<(), Box<dyn Error>> {
        // 1. Validate Comment contents.
        // 2. Connect to database.
        // 3. Query for comments LIKE the comment provided. 
        // 4. Return array of comments to user.
        todo!();
    }

    fn update_comment(&self, comment: Comment) -> Result<(), Box<dyn Error>> {
        // 1. Validate Comment contents.
        // 2. Connect to database.
        // 3. Submit update to databse.
        // 4. Return status of update.
        todo!();
    }
    
    fn delete_comment(&self, comment: Comment) -> Result<(), Box<dyn Error>> {
        // 1. Validate Comment contents.
        // 2. Connect to database.
        // 3. Flip visibility to 0. Do not actually delete the comment.
        // 4. Return status of deletion.
        todo!();
    }

}
