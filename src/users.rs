pub mod models {
    use bcrypt::verify;
    use diesel::prelude::*;
    use diesel::PgConnection;

    use crate::{graphql_schema, schema::users};

    pub fn create(conn: &PgConnection, input: &graphql_schema::NewUser) -> Result<(), String> {
        let hashed_pwd = hash_password(input.password.as_str());

        let result = diesel::insert_into(users::table)
            .values((
                users::username.eq(input.username.as_str()),
                users::password.eq(hashed_pwd.as_str()),
            ))
            .execute(conn);

        match result {
            Ok(_) => Ok(()),
            Err(error) => Err(error.to_string()),
        }
    }

    pub fn authenticate(conn: &PgConnection, login: &graphql_schema::Login) -> bool {
        use crate::schema::users::dsl::*;

        match users
            .select(password)
            .filter(username.eq(login.username.as_str()))
            .get_result::<String>(conn)
        {
            Ok(hashed_pwd) => check_password_hash(&login.password, &hashed_pwd),
            _ => false,
        }
    }

    pub fn get_userid_by_username(conn: &PgConnection, name: &str) -> Result<i32, String> {
        use crate::schema::users::dsl::*;

        let found = users
            .select(id)
            .filter(username.eq(name))
            .get_result::<i32>(conn);
        match found {
            Ok(user_id) => Ok(user_id),
            Err(error) => Err(error.to_string()),
        }
    }

    fn hash_password(password: &str) -> String {
        bcrypt::hash(password, 10).unwrap()
    }

    pub fn check_password_hash(password: &str, hash: &str) -> bool {
        verify(password, hash).unwrap()
    }
}
