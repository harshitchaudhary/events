use rocket::*;
use serde::{Deserialize, Serialize};
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use rocket::response;
use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response};
use rocket_contrib::json;
use rocket_contrib::uuid::Uuid;
use uuid::Uuid as uid;
use crate::data::db::{User, InsertableUser, ResponseUser, UserPassword};
use crate::data::mysql_connection::Conn;
use diesel::prelude::*;
use crate::schema::users;
use crypto::digest::Digest;
use crypto::sha3::Sha3;


#[derive(Insertable, PartialEq, Debug)]
#[table_name = "users"]
struct NewUser {
    id: i64,
    username: String,
    uuid: String,
    name: String,
    email: String,
    password: String,
    phone: i64,
    salt: String,
    game_id: String
}


#[derive(Debug)]
pub struct ApiResponse {
    status: Status,
    message: JsonValue,
}

impl ApiResponse {
    pub fn ok(message: JsonValue) -> Self {
        ApiResponse {
            status: Status::Ok,
            message: message,
        }
    }

    pub fn err(message: JsonValue) -> Self {
        ApiResponse {
            status: Status::InternalServerError,
            message: message,
        }
    }

    pub fn internal_err() -> Self {
        ApiResponse {
            status: Status::InternalServerError,
            message: json!("Internal server error"),
        }
    }
 
}



impl<'r> Responder<'r> for ApiResponse {

    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.message.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}
                  

#[get("/users")]
pub fn user_list_rt(connection: Conn) -> ApiResponse {
    use crate::schema::users::dsl::*;
    let user_coll = users.filter(name.is_not_null())
                    .load::<User>(&*connection)
                    .expect("Error loading users");

    ApiResponse::ok(json!(user_coll))
}


#[post("/users", format="json", data="<user>")]
pub fn new_user_rt(connection: Conn, user: Json<InsertableUser>) -> ApiResponse {
    use crate::schema::users::dsl::*;
    let new_uuid = uid::new_v4();
    let new_user = NewUser {
        id: 0,
        name: user.name.to_string(),
        username: user.username.to_string(),
        password: hashed_password(&user.password.to_string()),
        uuid: new_uuid.to_string(),
        email: user.email.to_string(),
        game_id: user.game_id.to_string(),
        salt: uid::new_v4().to_string(),
        phone: user.phone,
    };
    let results = diesel::insert_into(crate::schema::users::table)
        .values(new_user)
        .execute(&*connection)
        .unwrap();

    ApiResponse::ok(json!(format!("User saved successfully with uuid {}", new_uuid)))
}


#[get("/users/<uuid>")] 
pub fn info_user_rt(connection: Conn, uuid: String)  -> ApiResponse {
    use crate::schema::users::dsl::*;
    let user_coll = users.filter(uuid.eq(uuid))
                    .first::<User>(&*connection)
                    .expect("Error loading user");
    ApiResponse::ok(json!(user_coll))
}


#[put("/users/<uuid>", format="json", data="<user>")] 
pub fn update_user_rt(connection: Conn, user: Json<InsertableUser>, uuid: String) -> ApiResponse {
    use crate::schema::users::dsl::*;
    let updated_row = diesel::update(users.filter(uuid.eq(uuid)))
        .set( (name.eq(user.name.to_string()), username.eq(user.username.to_string())) )
        .execute(&*connection)
        .unwrap();

    ApiResponse::ok(json!(updated_row))    
}




#[delete("/users/<uuid>")]   
pub fn delete_user_rt(connection: Conn, uuid: String) -> ApiResponse {
    use crate::schema::users::dsl::*;
    let user_row = users.filter(uuid.eq(uuid))
                    .first::<User>(&*connection)
                    .expect("Error loading user");

    let results = diesel::delete(users.filter(id.eq(user_row.id)))
        .execute(&*connection)
        .unwrap();

    ApiResponse::ok(json!(results))

}

fn hashed_password(password: &String) -> String {
    let mut hashed = Sha3::sha3_256();
    hashed.input_str(password);

    return hashed.result_str();
}
