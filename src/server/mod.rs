pub mod route;

use actix_web::{App, HttpServer};
use listenfd::ListenFd;

pub async fn start(bind_addr: &str) -> std::io::Result<()> {
  let mut listenfd = ListenFd::from_env();
  let mut server = HttpServer::new(|| App::new().configure(route::routes));
  server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
    server.listen(l)?
  } else {
    server.bind(bind_addr)?
  };

  server.run().await
}
