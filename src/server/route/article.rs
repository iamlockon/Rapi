use actix_web::{web, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
  title: String,
  id: u32,
}

async fn article_detail<'a>(info: web::Path<Info>) -> impl Responder {
  format!("Title: {} \nID: {}", info.title, info.id)
}


pub fn route(cfg: &mut web::ServiceConfig) {
  cfg.service(web::resource("/article/{title}/{id}").route(web::get().to(article_detail)));
}