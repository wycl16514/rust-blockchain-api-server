pub mod apiserver;
use crate::apiserver::ApiServer;
pub mod blockchain;
pub mod wallet;
use actix_web::middleware::Logger;

#[actix_web::main]
async fn main() {
    env_logger::init();
    let server = ApiServer::new(5000);
    server.run().await;
}
