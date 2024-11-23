use std::sync::Arc;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::{Root};
use surrealdb::{Response, Surreal};
use log::info;
use crate::definitions::{BodyUser, User};

#[derive(Clone)]
pub struct DatabaseManager {
    database: Arc<Surreal<Client>>,
}

#[derive(Deserialize, Serialize)]
pub struct PaginationParams {
    pub(crate) page: Option<i64>,
    pub(crate) per_page: Option<i64>,
}

impl DatabaseManager {
    pub(crate) async fn init(db_host: String, db_name: String, db_pass: String, db_database: String, db_namespace: String) -> surrealdb::Result<Self> {
        info!("Connecting to database {}@{} with user {}...", db_database, db_host, db_name);
        // Connect to the database
        let db = Surreal::new::<Ws>(db_host).await?;

        // Select the namespace and database
        //TODO Do we also want a env var for the namespace?
        db.use_ns(db_namespace).use_db(db_database).await?;

        //TODO support more than just root auth
        db.signin(Root {
            username: &db_name,
            password: &db_pass,
        }).await?;

        info!("Connecting successful!");
        info!("Initializing database...");

        //TODO Initialize the tables, the relations and the events needed for the application
        let database = Arc::new(db);
        Ok(Self { database })
    }

    pub fn get_database(&self) -> Arc<Surreal<Client>> {
        self.database.clone()
    }

    pub async fn query(&self, query: String, bindings: impl Serialize + 'static) -> surrealdb::Result<Response> {
        self.database.query(query).bind(bindings).await
    }

    pub async fn fetch_users(&self) -> surrealdb::Result<Vec<User>> {
        let users: Vec<User> = self.database
            .query("SELECT * FROM user ORDER BY name ASC")
            .await?
            .take(0)?;

        Ok(users)
    }

    pub async fn delete_user(&self, id: String) -> surrealdb::Result<Option<User>> {
        let deleted: Option<User> = self.database.delete(("user", id)).await?;
        Ok(deleted)
    }

    pub async fn fetch_user(&self, name_or_email: String) -> surrealdb::Result<Option<User>> {
        let user: Vec<User> = self.database
            .query("SELECT * FROM user WHERE name = $name OR email = $name OR id = type::thing(\"user\", $name) LIMIT 1")
            .bind(("name", name_or_email))
            .await?
            .take(0)?;

        Ok(user.into_iter().nth(0))
    }

    pub async fn add_user(&self, user: BodyUser) -> surrealdb::Result<Vec<User>> {
        self.database
            .insert("user")
            .content(user)
            .await
    }

    pub async fn update_user(&self, user: &User) -> surrealdb::Result<Option<User>> {
        self.database
            .update(("user", user.id.to_string()))
            .merge(BodyUser {
                id: Some(user.id.clone()),
                name: Some(user.name.clone()),
                admin: Some(user.admin),
                email: Some(user.email.clone()),
                password: Some(user.password.clone()),
                firstname: user.firstname.clone(),
                lastname: user.lastname.clone()
            })
            .await
    }
}