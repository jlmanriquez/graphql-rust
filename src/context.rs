use crate::db::Pool;

pub struct GraphQLContext {
    pub pool: Pool,
}

// This impl allows us to pass in GraphQLContext as the Context for GraphQL
// objects
impl juniper::Context for GraphQLContext {}