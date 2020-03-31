use actix_web::{web, guard, Responder};
use std::sync::Mutex;

pub struct AppIndexState {
  app_name: String,
  req_id: Mutex<u64>
}

mod user;
mod article;

async fn index(data: web::Data<AppIndexState>) -> impl Responder {
  let mut id = data.req_id.lock().unwrap();
  *id += 1;
  format!("{} service by rust... {}", &data.app_name, *id)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
  cfg.service(web::resource("/").data(AppIndexState {
    app_name: String::from("Rapi"),
    req_id: Mutex::new(0)
  }).to(index))
  .service(
    web::scope("/api")
      .guard(guard::Header("Content-Type", "application/json"))
      .configure(user::route)
      .configure(article::route)
  );
}
