use diesel::prelude::*;
use crate::models::{User, NewUser};
use crate::schema::users;

pub fn establish_connection() -> PgConnection {
    // Replace with actual connection setup code
}

pub fn create_user(conn: &PgConnection, username: &str, password: &str) -> Result<User, diesel::result::Error> {
    let new_user = NewUser {
        username,
        password_hash: password, // Ensure you hash the password before saving
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}

pub fn get_user_by_username(conn: &PgConnection, username: &str) -> Result<User, diesel::result::Error> {
    users::table.filter(users::username.eq(username))
        .first(conn)
}














// use diesel::prelude::*;
// use diesel::sqlite::SqliteConnection;
// use crate::models::{User, NewUser};
// use crate::schema::users;

// pub fn establish_connection() -> SqliteConnection {
//     use dotenv::dotenv;
//     use std::env;

//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
// }

// pub fn create_user(conn: &SqliteConnection, username: &str, password: &str) -> Result<User, diesel::result::Error> {
//     use crate::schema::users::dsl::*;

//     let password_hash = crate::auth::hash_password(password).unwrap();

//     let new_user = NewUser {
//         username,
//         password_hash,
//     };

//     diesel::insert_into(users)
//         .values(&new_user)
//         .execute(conn)?;

//     users.filter(username.eq(username))
//         .first(conn)
// }

// pub fn get_user_by_username(conn: &SqliteConnection, username: &str) -> Result<User, diesel::result::Error> {
//     use crate::schema::users::dsl::*;

//     users.filter(username.eq(username))
//         .first(conn)
// }
