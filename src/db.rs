use diesel::{r2d2::ConnectionManager, PgConnection};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_pool(url: &str) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("could not build connection pool")
}
