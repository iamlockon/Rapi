pub mod route;
use actix_web::{App, HttpServer};
use listenfd::ListenFd;

use actix_web::middleware::Logger;
use env_logger::Env;

pub async fn start(bind_addr: &str) -> std::io::Result<()> {
  let mut listenfd = ListenFd::from_env();
  env_logger::from_env(Env::default().default_filter_or("info")).init();
  let mut server = HttpServer::new(|| App::new().wrap(Logger::default()).configure(route::routes));
  server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
    server.listen(l)?
  } else {
    server.bind(bind_addr)?
  };

  server.run().await
}
