use actix_web::{web, Responder};
use actix_web::web::{Json};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::error::ApiError;

#[derive(Deserialize)]
struct Info {
  name: String,
  id: u32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UserResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Deserialize)]
struct Meta {
  tags: Vec<u32>
}

async fn user_detail(info: web::Path<Info>) -> Result<Json<UserResponse>, ApiError> {
  let name = &info.name;
  Err(ApiError::InternalError)
  // Ok(Json(UserResponse{
  //   id: Uuid::new_v4(),
  //   first_name: String::from(name),
  //   last_name: String::from(name),
  //   email: String::from("fff")
  // }))
}

async fn update_user(info: web::Json<Meta>) -> impl Responder {
  format!("Tags: {:#?}", info.tags)
}


pub fn route(cfg: &mut web::ServiceConfig) {
  cfg.service(web::resource("/user/{name}/{id}").route(web::get().to(user_detail)));
  cfg.service(web::resource("/user/{id}").route(web::post().to(update_user)));
}