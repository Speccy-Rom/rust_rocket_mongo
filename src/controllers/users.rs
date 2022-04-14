use rocket::{
    http::Status,
    serde::json::{Json, Value},
    State,
};

use crate::{
    db::MongodbBackend,
    models::user::NewUser,
    secure::guards::{AdminGuard, UserGuard},
};

#[get("/my")]
pub async fn get_current_user(guard: UserGuard) -> (Status, Value) {
    // guard already contains current user under the provided api key
    super::generic_response(Ok(guard.0.inner))
}

#[post("/", format = "json", data = "<new_user>")]
pub async fn create_user(
    guard: AdminGuard,
    new_user: Json<NewUser>,
    backend: &State<MongodbBackend>,
) -> (Status, Value) {
    super::generic_response(
        backend
            .insert_user(
                new_user.to_owned(),
                guard.ip.to_string(),
                guard.email.clone(),
            )
            .await,
    )
}

#[get("/")]
pub async fn get_all_users(_guard: AdminGuard, backend: &State<MongodbBackend>) -> (Status, Value) {
    super::generic_response(backend.get_all_users().await)
}

#[put("/", format = "json", data = "<user>")]
pub async fn update_user(
    _guard: AdminGuard,
    user: Json<NewUser>,
    backend: &State<MongodbBackend>,
) -> (Status, Value) {
    super::generic_response(backend.update_user(user.0).await)
}

#[delete("/<email>")]
pub async fn delete_user(
    _guard: AdminGuard,
    email: String,
    backend: &State<MongodbBackend>,
) -> (Status, Value) {
    super::generic_response(backend.delete_user(email).await)
}
