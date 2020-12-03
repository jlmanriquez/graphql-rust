use juniper::FieldResult;
use juniper::RootNode;

use juniper::{GraphQLInputObject, GraphQLObject};

use crate::{context::GraphQLContext, links};

#[derive(GraphQLObject)]
pub struct Link {
    pub id: i32,
    pub title: String,
    pub address: String,
    pub user: Option<User>,
}

#[derive(GraphQLObject)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(GraphQLObject)]
pub struct Query {
    pub links: Vec<Link>,
}

#[derive(GraphQLInputObject)]
pub struct NewLink {
    pub title: String,
    pub address: String,
}

#[derive(GraphQLInputObject)]
pub struct RefreshTokenInput {
    pub token: String,
}

#[derive(GraphQLInputObject)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(GraphQLInputObject)]
pub struct Login {
    pub username: String,
    pub password: String,
}

pub struct QueryRoot;

#[juniper::object(Context = GraphQLContext)]
impl QueryRoot {
    fn links(context: &GraphQLContext) -> FieldResult<Vec<Link>> {
        let rows = links::models::get_all(&context.pool.get().unwrap())?;
        let all_links = rows
            .iter()
            .map(|l| Link {
                id: l.id,
                title: l.title.to_owned().unwrap_or(String::from("")),
                address: l.address.to_owned().unwrap_or(String::from("")),
                user: None,
            })
            .collect::<Vec<_>>();

        Ok(all_links)
    }
}

pub struct MutationRoot;

#[juniper::object(Context = GraphQLContext)]
impl MutationRoot {
    #[graphql(name = "createLink")]
    fn create_link(context: &GraphQLContext, input: NewLink) -> FieldResult<Link> {
        let new_id = links::models::save(&context.pool.get().unwrap(), &input).unwrap();

        Ok(Link {
            id: new_id,
            title: input.title,
            address: input.address,
            user: None,
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
