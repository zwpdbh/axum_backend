use async_graphql::{Context, Object, Schema};
use async_graphql::{EmptyMutation, EmptySubscription};

pub struct Query;

pub type ServiceSchema = Schema<Query, EmptyMutation, EmptySubscription>;

#[Object]
impl Query {
    /// hello is your first query. It just returns a static string for now
    async fn hello(&self, _ctx: &Context<'_>) -> &'static str {
        "Hello World"
    }
}
