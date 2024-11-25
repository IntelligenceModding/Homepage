use std::path::PathBuf;
use actix_web::{Error, get, HttpResponse, patch, put, Scope, web, post, delete};
use actix_web::web::Json;
use futures_util::StreamExt;
use log::error;
use crate::definitions::{BodyPost, BodyUser, Post, User};
use crate::storage::database_manager::DatabaseManager;
use crate::storage::storage_manager::StorageManager;

pub fn blog_service() -> Scope {
    web::scope("/api/v1/posts")
        .service(posts_get)
        .service(post_get)
        .service(post_delete)
        .service(post_post)
}

#[get("")]
async fn posts_get(
    user: User,
    database_manager: web::Data<DatabaseManager>) -> Result<HttpResponse, Error> {

    if !user.admin {
        return Ok(HttpResponse::Unauthorized().finish())
    }

    let posts = match database_manager.fetch_posts().await {
        Ok(posts) => posts,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };
    Ok(HttpResponse::Ok().json(posts))
}

#[get("/{postId}")]
async fn post_get(
    path: web::Path<String>,
    database_manager: web::Data<DatabaseManager>) -> Result<HttpResponse, Error> {

    let post_id = path.into_inner();

    let found_post: Option<Post> = match database_manager.fetch_post(post_id).await {
        Ok(found_post) => {
            found_post
        }
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    match found_post {
        Some(found_user) => {
            Ok(HttpResponse::Ok().json(found_user))
        }
        None => {
            Ok(HttpResponse::NotFound().finish())
        }
    }
}

#[delete("/{postId}")]
async fn post_delete(
    user: User,
    path: web::Path<String>,
    database_manager: web::Data<DatabaseManager>) -> Result<HttpResponse, Error> {

    let post_id = path.into_inner();

    if !user.admin {
        return Ok(HttpResponse::Unauthorized().finish())
    }

    match database_manager.delete_post(post_id).await {
        Ok(_) => {
            Ok(HttpResponse::Ok().finish())
        },
        Err(_) => {
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[post("")]
async fn post_post(
    user: User,
    body: Json<BodyPost>,
    database_manager: web::Data<DatabaseManager>) -> Result<HttpResponse, Error> {

    if !user.admin {
        return Ok(HttpResponse::Unauthorized().finish())
    }

    let post = body.into_inner();

    match database_manager.add_post(post).await {
        Ok(_) => {
        }
        Err(err) => {
            error!("Could not post post {err}");
            return Ok(HttpResponse::InternalServerError().finish())
        }
    }

    Ok(HttpResponse::Ok().finish())
}