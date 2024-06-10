use async_graphql::{Context, Object, Result};
use entity::async_graphql;
use entity::user;
use service::Query;

use crate::db::Database;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_note_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<user::Model>> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();

        Ok(Query::find_user_by_id(conn, id)
            .await
            .map_err(|e| e.to_string())?)
    }
    async fn hello(&self, _ctx: &Context<'_>) -> &'static str {
        "Hello World"
    }
}
