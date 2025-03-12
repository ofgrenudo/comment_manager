use chrono::Utc;
use sha256::digest;
use uuid::Uuid;

use crate::{Comment, CommentError};

pub fn comment(ip: String, username: String, comment: String) -> Result<Comment, CommentError> {
    let sha_ip = digest(&ip.replace(":", ""));

    let mut new_comment = Comment {
        id: Uuid::new_v4(),
        ip: sha_ip.to_string(),
        username: username,
        comment: comment,
        timestamp: Utc::now().to_string(),
        visible: 1,
        post_url: "null".to_string(),
    };

    if new_comment.username.len() > 500 {
        new_comment.username = "username_too_long".to_string();
        new_comment.visible = 0;
    } else if new_comment.ip.len() > 10000 {
        new_comment.ip = "ip_too_long".to_string();
        new_comment.visible = 0;
    } else if new_comment.comment.len() > 10000 {
        new_comment.comment = "comment_too_long".to_string();
        new_comment.visible = 0;
    }

    let connection = sqlite::open("comments.db").unwrap();

    // TODO: Extrapulate to a query builder function
    let query = format!(
        "CREATE TABLE IF NOT EXISTS comments (
            id TEXT NOT NULL PRIMARY KEY,
            ip TEXT,
            username TEXT NOT NULL,
            comment TEXT NOT NULL,
            timestamp TEXT,
            visible INT NOT NULL
        );
        INSERT INTO comments (id, ip, username, comment, timestamp, visible) 
        VALUES ('{}', '{}', '{}', '{}', '{}', {});",
        new_comment.id,
        new_comment.ip,
        new_comment.username,
        new_comment.comment,
        new_comment.timestamp,
        new_comment.visible
    );

    connection.execute(query).unwrap();

    Ok(new_comment)
}

pub fn comment_on_post(
    ip: String,
    username: String,
    comment: String,
    url: String,
) -> Result<Comment, CommentError> {
    let sha_ip = digest(&ip.replace(":", ""));

    let mut new_comment = Comment {
        id: Uuid::new_v4(),
        ip: sha_ip.to_string(),
        username: username,
        comment: comment,
        timestamp: Utc::now().to_string(),
        visible: 1,
        post_url: url.to_string(),
    };

    // Looks way better than it did before. But since im just modifying the comment, I wonder if I can just get rid of the `-> Result<Comment, CommentError>` and just return `-> Comment`
    if new_comment.username.len() > 500 {
        new_comment.username = "username_too_long".to_string();
        new_comment.visible = 0;
    } else if new_comment.ip.len() > 10000 {
        new_comment.ip = "ip_too_long".to_string();
        new_comment.visible = 0;
    } else if new_comment.comment.len() > 10000 {
        new_comment.comment = "comment_too_long".to_string();
        new_comment.visible = 0;
    }

    let connection = sqlite::open("comments.db").unwrap();

    let query = format!(
        "CREATE TABLE IF NOT EXISTS comments (
            id TEXT NOT NULL PRIMARY KEY,
            ip TEXT,
            username TEXT NOT NULL,
            comment TEXT NOT NULL,
            timestamp TEXT,
            visible INT NOT NULL,
            post_url TEXT
        );
        INSERT INTO comments (id, ip, username, comment, timestamp, visible, post_url) 
        VALUES ('{}', '{}', '{}', '{}', '{}', {}, '{}');",
        new_comment.id,
        new_comment.ip,
        new_comment.username,
        new_comment.comment,
        new_comment.timestamp,
        new_comment.visible,
        new_comment.post_url
    );

    connection.execute(query).unwrap();

    Ok(new_comment)
}
