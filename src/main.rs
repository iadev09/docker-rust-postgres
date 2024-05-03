use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use crate::user::User;

mod migrate;
mod user;

extern crate dotenv;

use dotenv::dotenv;
use tokio_postgres::NoTls;


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
pub async fn user_create(
    user: web::Json<User>,
    pool: web::Data<Pool>,
) -> HttpResponse {
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

macro_rules! dbg {
    ($($x:tt)*) => {
        {
            #[cfg(debug_assertions)]
            {
                std::dbg!($($x)*)
            }
            #[cfg(not(debug_assertions))]
            {
                ($($x)*)
            }
        }
    }
}


fn address() -> String {
    std::env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:8000".into())
}


fn db_config() -> Config {
    let mut cfg = Config::new();

    if let Ok(host) = std::env::var("PG_HOST") {
        cfg.host = Some(host);
    }
    if let Ok(dbname) = std::env::var("PG_DBNAME") {
        cfg.dbname = Some(dbname);
    }
    if let Ok(user) = std::env::var("PG_USER") {
        cfg.user = Some(user);
    }
    if let Ok(password) = std::env::var("PG_PASSWORD") {
        cfg.password = Some(password);
    }

    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    cfg
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    dotenv().ok();

    dbg!(".env loaded");

    let address = address();

    let pool = db_config()
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .unwrap();

    dbg!("pool created");

    migrate::up(&pool).await;

    dbg!("db migrated");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(user_list)
            .service(user_create)
    }).bind(&address)?
        .run()
        .await
}