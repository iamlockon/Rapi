mod server;
mod config;

#[actix_rt::main]
async fn main() {
    server::start(config::server::BIND_ADDR).await.expect("error");
}
