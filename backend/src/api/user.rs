use actix_web::{get, post, web, HttpResponse};
use names::{Generator, Name};
use serde::Deserialize;

use crate::db::util::DbPool;
use crate::extractors::auth::AuthenticatedUser;
use crate::model::user::User;
use crate::server_error::ServerError;

#[get("/user/{uid}")]
pub async fn get_user(
    uid: web::Path<String>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServerError> {
    let user = User::find_by_uid(pool.get_ref(), uid.as_ref()).await?;
    Ok(HttpResponse::Ok().json(user))
}

#[derive(Deserialize)]
pub struct NewUserBody {
    username: String,
}

fn random_guest_name() -> String {
    let mut generator = Generator::with_naming(Name::Numbered);
    generator.next().unwrap()
}

#[post("/user")]
pub async fn create_user(
    user: web::Json<NewUserBody>,
    auth_user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServerError> {
    let user = User::new(&auth_user.uid, &user.username, false)?;
    user.insert(&pool).await?;
    Ok(HttpResponse::Created().json(user))
}

#[post("/guest-user")]
pub async fn create_guest_user(
    auth_user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServerError> {
    let user = User::new(&auth_user.uid, &random_guest_name(), true)?;
    user.insert(&pool).await?;
    Ok(HttpResponse::Created().json(user))
}
