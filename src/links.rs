pub mod models {
    use crate::{graphql_schema, schema::links};
    use diesel::prelude::*;
    use diesel::PgConnection;

    #[derive(diesel::Queryable)]
    pub struct Link {
        pub id: i32,
        pub title: Option<String>,
        pub address: Option<String>,
        #[column_name = "userid"]
        pub user_id: Option<i32>,
    }

    pub fn save(conn: &PgConnection, input: &graphql_schema::NewLink) -> Result<i32, &'static str> {
        let result = diesel::insert_into(links::table)
            .values((
                links::title.eq(input.title.as_str()),
                links::address.eq(input.address.as_str()),
                links::userid.eq(input.user_id),
            ))
            .returning(links::id)
            .get_result::<i32>(conn);

        match result {
            Ok(link_id) => Ok(link_id),
            Err(_) => Err("error inserted Link"),
        }
    }

    pub fn get_all(conn: &PgConnection) -> Result<Vec<Link>, &'static str> {
        match links::dsl::links.load::<Link>(conn) {
            Ok(links) => Ok(links),
            Err(_) => Err("error geting all links"),
        }
    }
}
