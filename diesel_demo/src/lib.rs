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

pub fn create_post(title: &str, body: &str) {
    let connection = &mut establish_connection();

    let new_post = NewPost { title, body };

    let _post: Post = diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(connection)
        .expect("Error saving new post");
}

pub fn show_post() {
    use schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results: Vec<Post> = posts
        .filter(published.eq(false))
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
