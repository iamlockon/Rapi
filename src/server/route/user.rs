use actix_web::{web, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
  name: String,
  id: u32,
}

#[derive(Deserialize)]
struct Meta {
  tags: Vec<u32>
}

async fn user_detail(info: web::Path<Info>) -> impl Responder {
  format!("Username: {} {}", info.name, info.id)
}

async fn update_user(info: web::Json<Meta>) -> impl Responder {
  format!("Tags: {:#?}", info.tags)
}


pub fn route(cfg: &mut web::ServiceConfig) {
  cfg.service(web::resource("/user/{name}/{id}").route(web::get().to(user_detail)));
  cfg.service(web::resource("/user/{id}").route(web::post().to(update_user)));
}