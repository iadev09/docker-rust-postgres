extern crate dotenv;
use dotenv::dotenv;

use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use deadpool_postgres::Pool;

mod db;
mod migrate;
mod user;

use crate::user::User;
use db::create_pool;

#[get("/users")]
async fn user_list(pool: web::Data<Pool>) -> HttpResponse {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };

    match user::User::all(&**client).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(err) => {
            log::debug!("unable to fetch users: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to fetch users");
        }
    }
}

#[post("/users")]
pub async fn user_create(user: web::Json<User>, pool: web::Data<Pool>) -> HttpResponse {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };

    match user::User::create(&**client, &user).await {
        Ok(created) => HttpResponse::Ok().json(created),
        Err(err) => {
            log::debug!("unable to create user: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to create user");
        }
    }
}

fn address() -> String {
    return std::env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:8000".into());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();

    let address = address();
    let pool: Pool = create_pool();

    migrate::up(&pool).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(user_list)
            .service(user_create)
    })
    .bind(&address)?
    .run()
    .await
}
