use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, put,
    web::Data,
    web::Json,
    web::Path,
    HttpResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserId {
    id: i32,
}

// pub async fn get_user(user_id: Path<UserId>, body:Json<BodyStruct>) -> Json<String> {}

#[get("api/users/{id}")]
pub async fn get_user(id: Path<UserId>) -> Json<UserId> {
    Json(id.into_inner())
}
