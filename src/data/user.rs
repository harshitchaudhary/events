#[derive(Queryable)]
pub struct User {
    pub id: i64,
    pub uuid: String,
    pub name: String,
    pub username: String,
    pub phone: i64,
    pub wallet: i32,
    pub game_id: String,
    pub verification: i8,
}
