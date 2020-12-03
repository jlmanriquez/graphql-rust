pub mod models {
    extern crate bcrypt;

    use bcrypt::verify;
    use diesel::prelude::*;
    use diesel::PgConnection;
    use schema::users;

    use crate::{graphql_schema::NewUser, schema};

    pub fn create(conn: &PgConnection, input: &NewUser) {
        let hashed_pwd = hash_password(input.password.as_str());

        diesel::insert_into(users::table)
            .values((
                users::username.eq(input.username.as_str()),
                users::password.eq(hashed_pwd.as_str()),
            ))
            .execute(conn)
            .unwrap_err();
    }

    pub fn get_userId_by_username() {}

    fn hash_password(password: &str) -> String {
        bcrypt::hash(password, 14).unwrap()
    }

    pub fn check_password_hash(password: &str, hash: &str) -> bool {
        verify(password, hash).unwrap()
    }
}
