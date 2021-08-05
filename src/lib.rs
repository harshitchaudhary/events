#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_attributes)]
#[macro_use]
extern crate diesel;
extern crate dotenv;

use rocket::*;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::helmet::SpaceHelmet;

mod routes;
pub mod data;
pub mod schema;


pub fn rocket_builder() -> rocket::Rocket {
    rocket::ignite()
        .attach(SpaceHelmet::default())
        .mount("/", routes![routes::ping::ping_fn])
        .mount("/api", routes![
            routes::user::user_list_rt,
            routes::user::new_user_rt,
            routes::user::info_user_rt,
            routes::user::update_user_rt,
            routes::user::delete_user_rt
        ])
        .mount("/files", StaticFiles::from("static/"))
        .manage(data::mysql_connection::init_pool())
}

