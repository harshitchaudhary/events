table! {
    users (id) {
        id -> Bigint,
        uuid -> Varchar,
        name -> Varchar,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        salt -> Varchar,
        phone -> Bigint,
        wallet -> Bigint,
        game_id -> Varchar,
        verification -> Tinyint,
    }
}