use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use crate::auth::{generate_jwt, verify_password};
use crate::database::{establish_connection, create_user, get_user_by_username};
use crate::models::User;

async fn register_user(req: web::Json<User>) -> impl Responder {
    let conn = establish_connection();
    match create_user(&conn, &req.username, &req.password) {
        Ok(user) => HttpResponse::Created().json(user),
        Err(_) => HttpResponse::InternalServerError().body("Error creating user"),
    }
}

async fn login_user(req: web::Json<User>) -> impl Responder {
    let conn = establish_connection();
    match get_user_by_username(&conn, &req.username) {
        Ok(user) => {
            match verify_password(&user.password_hash, &req.password) {
                Ok(is_valid) => {
                    if is_valid {
                        let token = generate_jwt(&user.username);
                        HttpResponse::Ok().json(token)
                    } else {
                        HttpResponse::Unauthorized().body("Invalid credentials")
                    }
                }
                Err(_) => HttpResponse::InternalServerError().body("Error verifying password"),
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
