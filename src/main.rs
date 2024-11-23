use std::env;
use std::io::{Error, ErrorKind};
use std::sync::Arc;
use actix_web::{App, HttpServer};
use actix_web::web::Data;
use dotenv::dotenv;
use crate::storage::database_manager::{DatabaseManager};
use crate::storage::s3_storage_manager::S3SystemStorage;
use crate::storage::storage_manager::StorageManager;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey};
use crate::auth::auth_service;

mod api { // Declare the 'api' module

    pub mod users;
}

mod storage { // Declare the 'storage' module
    pub mod storage_manager;
    pub mod s3_storage_manager;
    pub mod database_manager;
}

mod definitions;
mod auth;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    // For debugging purposes, we will eventually remove this.
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Initiate storage
    let file_path = get_env_var("FILE_PATH")?;

    let storage_manager = StorageManager::new(Arc::new(S3SystemStorage::new(file_path)));

    let db_host = get_env_var("DB_HOST")?;
    let db_name = get_env_var("DB_USER")?;
    let db_pass = get_env_var("DB_PASS")?;
    let db_database = get_env_var("DB_DATABASE")?;
    let db_namespace = get_env_var("DB_DATABASE")?;

    let jwt_secret = get_env_var("JWT_SECRET")?;

    let db_manager;
    match DatabaseManager::init(db_host, db_name, db_pass, db_database, db_namespace).await {
        Ok(manager) => {
            db_manager = manager;
        },
        Err(e) => {
            panic!("couldn't initialize database: {e}");
        }
    }

    HttpServer::new(move || {
        let auth_manager = auth::AuthManager::new(Algorithm::HS256, EncodingKey::from_secret(jwt_secret.as_ref()), DecodingKey::from_secret(jwt_secret.as_ref()));

        App::new()
            .app_data(Data::new(auth_manager))
            .app_data(Data::new(storage_manager.clone()))
            .app_data(Data::new(db_manager.clone()))

            .service(auth_service())
            .service(api::users::user_service())
    })
        .workers(2)
        .bind("0.0.0.0:6969")?
        .run()
        .await
}

fn get_env_var(key: &str) -> Result<String, Error> {
    env::var(key).map_err(|e| {
        Error::new(ErrorKind::InvalidInput, format!("couldn't interpret {key}: {e}"))
    })
}
