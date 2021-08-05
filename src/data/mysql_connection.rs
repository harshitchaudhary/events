#[macro_use]
use std::ops::Deref;
use std::env;

use dotenv::dotenv;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager, PoolError };

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

pub type MyPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MyPoolConnection = PooledConnection<ConnectionManager<MysqlConnection>>;
 

fn establish_connection(database_url: &str) -> Result<MyPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager)
} 

pub fn init_pool() -> MyPool {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

    establish_connection(&database_url).expect("Failed to created pool")

}

pub struct Conn(pub MyPoolConnection);

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, Self::Error> {
        let pool = request.guard::<State<MyPool>>()?;
        match pool.get() {
            Ok(database) => Outcome::Success(Conn(database)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}
impl Deref for Conn {
    type Target = MyPoolConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
