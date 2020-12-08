use crate::{db::Pool, graphql_schema};

pub struct GraphQLContext {
    pub pool: Pool,
    pub user: Option<graphql_schema::User>,
}

// This impl allows us to pass in GraphQLContext as the Context for GraphQL
// objects
impl juniper::Context for GraphQLContext {}