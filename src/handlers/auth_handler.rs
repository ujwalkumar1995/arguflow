use std::future::{ready, Ready};

use actix_identity::Identity;
use actix_web::{
    dev::Payload, web, Error, FromRequest, HttpMessage as _, HttpRequest, HttpResponse,
};
use diesel::prelude::*;
use serde::Deserialize;

use crate::{
    errors::ServiceError,
    data::models::{Pool, SlimUser, User},
};

use crate::handlers::register_handler;

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

// we need the same data
// simple aliasing makes the intentions clear and its more readable
pub type LoggedUser = SlimUser;

impl FromRequest for LoggedUser {
    type Error = Error;
    type Future = Ready<Result<LoggedUser, Error>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        if let Ok(identity) = Identity::from_request(req, pl).into_inner() {
            if let Ok(user_json) = identity.id() {
                if let Ok(user) = serde_json::from_str(&user_json) {
                    return ready(Ok(user));
                }
            }
        }

        ready(Err(ServiceError::Unauthorized.into()))
    }
}

pub fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
    argon2::verify_encoded_ext(hash, password.as_bytes(), register_handler::SECRET_KEY.as_bytes(), &[]).map_err(
        |err| {
            dbg!(err);
            ServiceError::Unauthorized
        },
    )
}

pub async fn logout(id: Identity) -> HttpResponse {
    id.logout();
    HttpResponse::NoContent().finish()
}

pub async fn login(
    req: HttpRequest,
    auth_data: web::Json<AuthData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = web::block(move || query(auth_data.into_inner(), pool)).await??;

    let user_string = serde_json::to_string(&user).unwrap();
    Identity::login(&req.extensions(), user_string).unwrap();

    Ok(HttpResponse::NoContent().finish())
}

pub async fn get_me(logged_user: LoggedUser) -> HttpResponse {
    HttpResponse::Ok().json(logged_user)
}
/// Diesel query
fn query(auth_data: AuthData, pool: web::Data<Pool>) -> Result<SlimUser, ServiceError> {
    use crate::data::schema::users::dsl::{email, users};

    let mut conn = pool.get().unwrap();

    let mut items = users
        .filter(email.eq(&auth_data.email))
        .load::<User>(&mut conn)?;

    if let Some(user) = items.pop() {
        if let Ok(matching) = verify(&user.hash, &auth_data.password) {
            if matching {
                return Ok(user.into());
            }
        }
    }
    Err(ServiceError::Unauthorized)
}
