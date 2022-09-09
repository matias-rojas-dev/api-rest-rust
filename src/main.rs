use actix_cors::Cors;
use actix_web::{http, middleware, App, HttpServer};
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use std::env;
use api_service::ApiService;

mod api_router;
mod api_service;

pub struct ServiceMananger {
    api: ApiService,
}

impl ServiceMananger {
    pub fn new(api: ApiService) -> Self {
        ServiceMananger { api }
    }
}

pub struct AppState {
    service_manager: ServiceMananger
}

fn main() {
    println!("API REST WITH RUST");
}

/* 
#[actix_rt::main]
async fn main () -> std::io::Result<()> {
    dotenv().ok();

    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();


}
*/