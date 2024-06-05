extern crate diesel;

use crate::models::Post;
use crate::schema::posts;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::NewPost;

use std::env;
pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn show_post() {
    use schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results: Vec<Post> = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}

pub fn create_post(title: &str, body: &str) {
    let connection = &mut establish_connection();

    let new_post = NewPost { title, body };

    let _post: Post = diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(connection)
        .expect("Error saving new post");
}

pub fn update_post(id: i32) {
    use schema::posts::dsl::{posts, published};

    let connection = &mut establish_connection();

    let post: Post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .returning(Post::as_returning())
        .get_result(connection)
        .unwrap();

    println!("Published post: {}", post.title);
}

pub fn select_post(id: i32) {
    use self::schema::posts::dsl::posts;

    let connection = &mut establish_connection();

    let post: Result<Option<Post>, diesel::result::Error> = posts
        .find(id)
        .select(Post::as_select())
        .first(connection)
        .optional();

    match post {
        Ok(Some(post)) => println!("Post with id: {} has a title: {}", post.id, post.title),
        Ok(None) => println!("Unable to find post {}", id),
        Err(_) => println!("An error occured while fetching post {}", id),
    }
}

pub fn delete_post(target: &str) {
    use self::schema::posts::dsl::*;

    let pattern = format!("%{}%", target);
    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
