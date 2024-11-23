use std::future::Future;
use std::pin::Pin;
use std::time::SystemTime;
use actix_web::{Error, error, FromRequest, get, HttpRequest, HttpResponse, post, Scope, web};
use actix_web::dev::Payload;
use actix_web::web::{Data, Json};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{encode, decode, Header as JwtHeader, Algorithm, Validation, EncodingKey, DecodingKey, errors::Result as JwtResult};
use serde::{Deserialize, Serialize};
use crate::definitions::User;
use crate::storage::database_manager::DatabaseManager;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize,          // Optional. Issued at (as UTC timestamp)
    iss: String,         // Optional. Issuer
    sub: String,         // Optional. Subject (whom token refers to)
}

#[derive(Clone)]
pub(crate) struct AuthManager {
    algorithm: Algorithm,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl AuthManager {
    pub(crate) fn new(algorithm: Algorithm, encoding_key: EncodingKey, decoding_key: DecodingKey) -> Self {
        AuthManager {
            algorithm,
            encoding_key,
            decoding_key,
        }
    }

    fn create_token(&self, claims: &Claims) -> JwtResult<String> {
        encode(&JwtHeader::new(self.algorithm), claims, &self.encoding_key)
    }

    fn validate_token(&self, token: String) -> JwtResult<Claims> {
        Ok(decode::<Claims>(&token, &self.decoding_key, &Validation::new(self.algorithm))?.claims)
    }
}

impl FromRequest for User {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();

        let auth_manager = req.app_data::<Data<AuthManager>>().unwrap().get_ref().clone();
        let database_manager = req.app_data::<Data<DatabaseManager>>().unwrap().get_ref().clone();

        Box::pin(async move {
            let bearer_auth = BearerAuth::from_request(&req, &mut Payload::None).await;
            match bearer_auth {
                Ok(auth) => {
                    let token = auth.token();

                    match auth_manager.validate_token(token.to_string()) {
                        Ok(claims) => {
                            match database_manager.fetch_user(claims.sub).await {
                                Ok(user) => {
                                    match user {
                                        Some(user) => Ok(user),
                                        None => Err(error::ErrorUnauthorized("User not found"))
                                    }
                                },
                                Err(_) => Err(error::ErrorUnauthorized("Failed to fetch user from database"))
                            }
                        },
                        Err(_) => Err(error::ErrorUnauthorized("Invalid token"))
                    }
                },
                Err(_) => Err(error::ErrorUnauthorized("Invalid token"))
            }
        })
    }
}

#[derive(Deserialize)]
struct LoginCredentials {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginToken {
    token: String
}

pub fn auth_service() -> Scope {
    web::scope("/api/v1/auth")
        .service(auth_me)
        .service(auth_login)
}

#[post("/login")]
async fn auth_login(
    login_credentials: Json<LoginCredentials>,
    database_manager: Data<DatabaseManager>,
    auth_manager: Data<AuthManager>,
) -> Result<HttpResponse, Error> {
    let user = database_manager.fetch_user(login_credentials.username.clone()).await;
    match user {
        Ok(user) => {
            match user {
                Some(user) => {
                    if user.validate_password(login_credentials.password.clone()) {
                        let iat = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as usize;
                        let claims = Claims {
                            exp: iat + (24 * 60 * 60),
                            iat,
                            iss: "intelligence".to_string(),
                            sub: user.id.to_string(),
                        };

                        match auth_manager.create_token(&claims) {
                            Ok(token) => Ok(HttpResponse::Ok().json(LoginToken { token })),
                            Err(_) => Err(error::ErrorInternalServerError("Failed to create token"))
                        }
                    } else {
                        Err(error::ErrorUnauthorized("Invalid credentials"))
                    }
                },
                None => Err(error::ErrorUnauthorized("Invalid credentials"))
            }
        },
        Err(_) => Err(error::ErrorInternalServerError("Failed to fetch user from database"))
    }
}

#[get("/me")]
async fn auth_me(user: User) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(user))
}