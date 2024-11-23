use std::path::PathBuf;
use actix_web::{Error, get, HttpResponse, patch, put, Scope, web, post, delete};
use actix_web::web::Json;
use futures_util::StreamExt;
use log::error;
use crate::definitions::{BodyUser, User};
use crate::storage::database_manager::DatabaseManager;
use crate::storage::storage_manager::StorageManager;

pub fn user_service() -> Scope {
    web::scope("/api/v1/users")
        .service(users_get)
        .service(user_get)
        .service(user_delete)
        .service(user_post)
        .service(user_exists)
        .service(user_patch)
        .service(user_picture_put)
        .service(user_picture_get)
}

#[get("")]
async fn users_get(
    user: User,
    database_manager: web::Data<DatabaseManager>) -> Result<HttpResponse, Error> {

    if !user.admin {
        return Ok(HttpResponse::Unauthorized().finish())
    }

    let users = match database_manager.fetch_users().await {
        Ok(users) => users,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };
    Ok(HttpResponse::Ok().json(users))
}

#[get("/{userId}")]
async fn user_get(
    user: User,
    path: web::Path<String>,
    database_manager: web::Data<DatabaseManager>) -> Result<HttpResponse, Error> {

    let user_id = path.into_inner();

    if !user.admin && !user.compare(&user_id) {
        return Ok(HttpResponse::Unauthorized().finish())
    }

    let found_user: Option<User> = match database_manager.fetch_user(user_id).await {
        Ok(found_user) => {
            found_user
        },
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    match found_user {
        Some(found_user) => {
            Ok(HttpResponse::Ok().json(found_user))
        }
        None => {
            Ok(HttpResponse::NotFound().finish())
        }
    }
}

#[delete("/{userId}")]
async fn user_delete(
    user: User,
    path: web::Path<String>,
    database_manager: web::Data<DatabaseManager>) -> Result<HttpResponse, Error> {

    let user_id = path.into_inner();

    if !user.admin {
        return Ok(HttpResponse::Unauthorized().finish())
    }

    match database_manager.delete_user(user_id).await {
        Ok(_) => {
            Ok(HttpResponse::Ok().finish())
        },
        Err(_) => {
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[post("")]
async fn user_post(
    user: User,
    body: Json<BodyUser>,
    database_manager: web::Data<DatabaseManager>) -> Result<HttpResponse, Error> {

    if !user.admin {
        return Ok(HttpResponse::Unauthorized().finish())
    }

    let user = body.into_inner(); // Extract the user from Json

    match database_manager.add_user(user).await {
        Ok(_) => {
        }
        Err(err) => {
            error!("Could not update user {err}");
            return Ok(HttpResponse::InternalServerError().finish())
        }
    }

    Ok(HttpResponse::Ok().finish())
}


#[get("/exists/{userId}")]
async fn user_exists(
    _user: User,
    path: web::Path<String>,
    database_manager: web::Data<DatabaseManager>) -> Result<HttpResponse, Error> {

    let user_id = path.into_inner();

    match database_manager.fetch_user(user_id).await {
        Ok(found_user) => {
            match found_user {
                Some(_) => {
                    Ok(HttpResponse::Ok().finish())
                }
                None => {
                    Ok(HttpResponse::NotFound().finish())
                }
            }
        },
        Err(_) => {
            Ok(HttpResponse::InternalServerError().finish())
        }
    }

}


#[patch("/{userId}")]
async fn user_patch(
    user: User,
    body: Json<BodyUser>,
    path: web::Path<String>,
    database_manager: web::Data<DatabaseManager>) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();

    if !user.admin && !user.compare(&user_id) {
        return Ok(HttpResponse::Unauthorized().finish())
    }

    let found_user: Option<User> = match database_manager.fetch_user(user_id).await {
        Ok(found_user) => found_user,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    match found_user {
        None => {
            Ok(HttpResponse::NotFound().finish())
        }
        Some(mut modified_user) => {
            // Apply partial updates based on the fields provided in the request body
            // Can we make this a bit more clean? Especially if we have more fields in the future, this is ass to maintain
            if let Some(name) = body.name.clone() {
                modified_user.name = name;
            }
            if let Some(admin) = body.admin.clone() {
                modified_user.admin = admin;
            }
            if let Some(email) = body.email.clone() {
                modified_user.email = email;
            }
            if let Some(password) = body.password.clone() {
                modified_user.password = password;
            }
            if let Some(firstname) = body.firstname.clone() {
                modified_user.firstname = Option::from(firstname);
            }
            if let Some(lastname) = body.lastname.clone() {
                modified_user.lastname = Option::from(lastname);
            }

            match database_manager.update_user(&modified_user).await {
                Ok(_) => {
                    // We use the body here since I do not want to send the password back in the response
                    //if it was not included in the request.
                    Ok(HttpResponse::Ok().json(body))
                }
                Err(err) => {
                    error!("Couldn't patch user {}", err);
                    Ok(HttpResponse::InternalServerError().finish())
                }
            }
        }
    }
}

#[put("/{userId}/image")]
async fn user_picture_put(
    user: User,
    mut payload: web::Payload,
    path: web::Path<String>,
    storage_manager: web::Data<StorageManager>) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();

    let mut bytes = Vec::new();

    if !user.admin && !user.compare(&user_id) {
        return Ok(HttpResponse::Unauthorized().finish())
    }

    while let Some(chunk) = payload.next().await {
        let chunk = chunk.expect("Error reading chunk");
        bytes.extend_from_slice(&chunk);
    }

    match storage_manager.put(&PathBuf::from(format!("userimages/{}", user_id)), &bytes).await {
        Ok(_) => {
            Ok(HttpResponse::Ok().finish())
        }
        Err(err) => {
            error!("Couldn't save picture {}", err);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[get("/{userId}/image")]
async fn user_picture_get(
    mut payload: web::Payload,
    path: web::Path<String>,
    storage_manager: web::Data<StorageManager>) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();

    let mut bytes = Vec::new();

    while let Some(chunk) = payload.next().await {
        let chunk = chunk.expect("Error reading chunk");
        bytes.extend_from_slice(&chunk);
    }

    let full_path = PathBuf::from(format!("userimages/{}", user_id));
    match storage_manager.get(&full_path).await {
        Ok(file_data) => {
            match file_data {
                Some(data) => {
                    Ok(HttpResponse::Ok()
                        .content_type("image/png")
                        .body(data))
                }
                None => {
                    Ok(HttpResponse::NotFound().finish())
                }
            }
        }
        Err(err) => {
            error!("Couldn't serve picture {}", err);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}