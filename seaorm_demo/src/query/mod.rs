use async_graphql::ComplexObject;
use async_graphql::Enum;
use async_graphql::SimpleObject;
use async_graphql::{Context, Object, Schema};
use async_graphql::{EmptyMutation, EmptySubscription};

#[derive(Debug, Enum, PartialEq, Eq, Clone, Copy)]

enum Role {
    Admin,
    User,
}

impl Default for Role {
    fn default() -> Self {
        Self::User
    }
}

// Define a complex object
#[derive(Debug, SimpleObject, Default)]
#[graphql(complex)]
struct User {
    role: Role,
    username: String,
    email: String,
    address: String,
    age: i32,
}

#[ComplexObject]
impl User {
    async fn users(&self) -> String {
        format!("{:?}", self)
    }
}

pub struct Query;

pub type ServiceSchema = Schema<Query, EmptyMutation, EmptySubscription>;

#[Object]
impl Query {
    /// hello is your first query. It just returns a static string for now
    async fn hello(&self, _ctx: &Context<'_>) -> &'static str {
        "Hello World"
    }
}
