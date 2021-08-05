use lazy_static;
use rocket::http::{ContentType, Status};
use bd_back::data::db::ResponseUser;

mod common;

#[test]
fn echo_test()  {
    let client = common::setup();
    let mut response = client.get("/ping").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("PONG!".into()));
}

#[test]
fn user_list_rt_test(){
    let client = common::setup();
    let mut response = client.get("/api/users").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let mut response_body = response.body_string().unwrap();
    response_body.retain(|c| !c.is_numeric());
    assert_eq!(response_body, "[]");

}


#[test]
fn new_user_rt_test(){
    let client = common::setup();
    let mut response = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
                "name" : "HarshitTest",
                "email" : "harshit.chaudhary+test@y7mail.com",
                "phone" : 9873990707,
                "username" : "lakkadbugga1",
                "game_id": "214356213",
                "password" : "jaiho"
            }"##)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let response_body = response.body_string().expect("Response Body");
    let user: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    assert_eq!(user.name, "HarshitTest");
    assert_eq!(user.email, "harshit.chaudhary+test@y7mail.com");
    assert_eq!(user.phone, 9873990707);
    assert_eq!(user.game_id, "214356213");
    assert_eq!(user.username, "lakkadbugga1");

}

#[test]
fn info_user_rt_test(){
    let client = common::setup();
    let mut response = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
                "name" : "HarshitTestInfo",
                "email" : "harshit.chaudhary+testInfo@y7mail.com",
                "phone" : 9873990607,
                "username" : "lakkadbugga2",
                "game_id": "214356212",
                "password" : "jaiho1"
            }"##)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let response_body = response.body_string().expect("Response Body");
    let new_user: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    let uuid = new_user.uuid;
    let mut response = client.get(format!("/api/users/{}", uuid)).dispatch();
    let response_body = response.body_string().expect("Response Body");
    let user: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    assert_eq!(user.name, "HarshitTestInfo");
    assert_eq!(user.email, "harshit.chaudhary+testInfo@y7mail.com");
    assert_eq!(user.phone, 9873990607);
    assert_eq!(user.game_id, "214356212");
    assert_eq!(user.username, "lakkadbugga2");
 

}
