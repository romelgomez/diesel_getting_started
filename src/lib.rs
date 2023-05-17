use self::models::{NewPost, Post};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_create_post() {
        // Establish connection to the test database
        let mut connection = establish_connection();

        let title = "Test Post";
        let body = "This is a test post.";

        // Create a new post
        let post = create_post(&mut connection, title, body);

        // Check that the post was created correctly
        assert_eq!(post.title, title);
        assert_eq!(post.body, body);

        // TODO:
        // Delete the created post after test
        // delete_post(&mut connection, &post.id);
    }
}
