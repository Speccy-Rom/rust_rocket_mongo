pub mod client;
pub mod user;

pub use user::{AdminGuard, UserGuard};

use derive_more::Deref;
use rocket::request::Request;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

use crate::{db::MongodbBackend, error::Error, models::user::User};

#[derive(Serialize, Deserialize, Deref)]
pub struct GuardedData<T> {
    #[deref]
    pub inner: T,
    pub ip: IpAddr,
}

pub(crate) async fn get_user_from_request(
    request: &Request<'_>,
    backend: &MongodbBackend,
) -> Result<GuardedData<User>, Error> {
    let api_key = request
        .headers()
        .get_one("x-api-key")
        .map(|header| header.trim())
        .ok_or(Error::UnauthenticatedUser)?;

    backend
        .get_user_from_api_key(api_key)
        .await
        .and_then(|user| {
            match (
                user.is_endpoint_allowed(request.uri().path().as_str()),
                request.client_ip(),
            ) {
                (true, Some(ip)) if user.is_ip_accessible(&ip.to_string()) => {
                    Ok(GuardedData { inner: user, ip })
                }
                _ => Err(Error::ForbiddenAccess),
            }
        })
}
