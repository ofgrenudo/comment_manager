use std::{
    error::Error,
    io,
    path::{self, Path},
};

use async_trait::async_trait;
use sqlite::Connection;
use uuid::Uuid;

use crate::{Comment, Database};

pub struct SQLite {
    pub connection_string: String,
    pub conn: Connection,
}

const DATABASE_INIT_STRING: &str = "CREATE TABLE IF NOT EXISTS comments (
                    id TEXT NOT NULL PRIMARY KEY,
                    ip TEXT,
                    username TEXT NOT NULL,
                    comment TEXT NOT NULL,
                    timestamp TEXT,
                    visible INT NOT NULL,
                    post_url TEXT
                );";

#[async_trait]
impl Database for SQLite {
    fn connect(connection_information: &str) -> Result<SQLite, Box<dyn Error>> {
        // 1. Create an instance of SQLite;
        let conn = Connection::open(&connection_information)?;
        let connection_information_modified = SQLite {
            conn,
            connection_string: connection_information.to_string(),
        };

        // 2 Initialize database.
        Database::initialize_database(&connection_information_modified)?;

        // 3. Return database connection.
        Ok(connection_information_modified)
    }

    fn initialize_database(&self) -> Result<(), Box<dyn Error>> {
        // 1. Connect to database.
        // NOTE Connection::open from the connect method will create the file if it does not exist!
        let database: &Connection = &self.conn;

        // 2. Initialize database.
        database.execute(DATABASE_INIT_STRING.to_string())?;

        // 3. Return status of intialization.
        Ok(())
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

    // NOTE: Not sure what im doing wrong here. Trying to return a specific error. this type of shit is why i just make a custom type, and return the error that way instead of using rust native shit.
    fn validate_comment(&self, comment: Comment) -> Result<(), Box<dyn Error>> {
        // 1. Validate comment contents.
        comment.into_iter().for_each(|item| {
            if item.len() > 10000 {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Comment is too long",
                )));
            } else {
                return Ok(());
            }
        });

        comment.into_iter().for_each(|item| {
            if item.is_empty() {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Item cannot be empty",
                )));
            }
        });

        // 2. Return status of validation.
        Ok(())
    }
}
