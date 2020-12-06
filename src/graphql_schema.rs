use juniper::RootNode;
use juniper::{FieldError, FieldResult};

use juniper::{GraphQLInputObject, GraphQLObject};

use crate::{context::GraphQLContext, jwt, links, users};

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

    #[graphql(name = "createUser")]
    fn create_user(context: &GraphQLContext, input: NewUser) -> FieldResult<String> {
        users::models::create(&context.pool.get().unwrap(), &input)?;
        let token = jwt::generate_token(input.username.as_str())?;
        Ok(token)
    }

    fn login(context: &GraphQLContext, input: Login) -> FieldResult<String> {
        let correct = users::models::authenticate(&context.pool.get().unwrap(), &input);
        if !correct {
            return Err(FieldError::from("wrong username or password"));
        }
        Ok(jwt::generate_token(&input.username)?)
    }

    #[graphql(name = "refreshToken")]
    fn refresh_token(context: &GraphQLContext, input: RefreshTokenInput) -> FieldResult<String> {
        let username = match jwt::parse_token(&input.token) {
            Ok(username) => username,
            _ => return Err(FieldError::from(String::from("access denied"))),
        };
        Ok(jwt::generate_token(&username)?)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
