-- Your SQL goes here
create table user (
    id bigint primary key auto_increment,
    uuid varchar(80) unique not null,
    name varchar(40) not null,
    username varchar(20) not null,
    password varchar(80) not null,
    phone bigint not null,
    wallet int null default 0,
    game_id varchar(20) null,
    verification tinyint default 0,
    created_at timestamp default current_timestamp, 
    updated_at timestamp default current_timestamp on update current_timestamp
);
