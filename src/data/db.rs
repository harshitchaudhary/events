use serde::{Deserialize, Serialize};
use uuid::Uuid;
use argon2;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use chrono::{NaiveDateTime, NaiveDate};
use diesel::dsl;



#[derive(Serialize, Queryable, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub uuid: String,
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub salt: String, 
    pub phone: i64,
    pub wallet: i64,
    pub game_id: String,
    pub verification: i8,
}

#[derive(Serialize, Deserialize, Debug, Clone)] 
pub struct InsertableUser{
    pub username: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone: i64,
    pub game_id: String
 
}


//impl User {
//    pub fn new(name: String, email: String, password: String, phone: i64, game_id: String, username: String) -> Self {
//        let salt: String = thread_rng()
//                .sample_iter(&Alphanumeric)
//                .map(char::from)
//                .take(20)
//                .collect();
//
//        let hashed_password = hash_password(&password, &salt);
//
//        User {
//            id: 0,
//            uuid: Uuid::new_v4().to_string(),
//            username,
//            name,
//            email,
//            phone,
//            game_id,
//            wallet: 0,
//            verification: 0,
//            password,
//            salt        
//        }
//    }
//
//    pub fn from_insertable(insertable: InsertableUser) -> Self {
//        User::new(insertable.name, insertable.email, insertable.password, insertable.phone, insertable.game_id, insertable.username)
//    }
//
//    pub fn match_password(&self, password: &String) -> bool {
//        argon2::verify_encoded(&self.password, password.as_bytes()).unwrap()
//    }
//
//    pub fn update_password(&mut self, password: &String) {
//        self.password = hash_password(password, &self.salt);
//    }
//
//    pub fn update_user(&mut self, name: &String, username: &String, email: &String, phone: &i64, game_id: &String)  {
//        self.name = name.to_string();
//        self.username = username.to_string();
//        self.email = email.to_string();
//        self.phone = *phone;
//        self.game_id = game_id.to_string();
//    }
//}

fn hash_password(password: &String, salt: &String) -> String {
    let config = argon2::Config::default();
    argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap()
}
 

#[derive(Serialize, Deserialize, Debug, Clone)] 
pub struct ResponseUser {
    pub uuid: String,
    pub name: String,
    pub username: String,
    pub email: String,
    pub game_id: String,
    pub phone: i64,
    pub wallet: i64,
}

impl ResponseUser {
    pub fn from_user(user: &User) -> Self {
        ResponseUser {
            uuid: user.uuid.to_string(),
            name: format!("{}", user.name),
            username: format!("{}", user.username),
            email: format!("{}", user.email),
            game_id: format!("{}", user.game_id),
            phone: user.phone,
            wallet: user.wallet
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)] 
pub struct UserPassword {
    pub password: String,
    pub new_password: Option<String>,
}
