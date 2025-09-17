#[macro_use]
extern crate rocket;
use std::net::{IpAddr, Ipv4Addr};

use rocket::{
    fs::{relative, FileServer},
    Config,
};

mod routes;
use routes::init;

mod db;

#[launch]
async fn rocket() -> _ {
    init().await.expect("Failed to startup db...");
    rocket::build()
        .configure(rocket::Config {
            port: std::env::var("ROCKET_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap(),
            address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),

            ..Config::default()
        })
        .mount("/", FileServer::from("./frontend/dist"))
        .mount(
            "/api",
            routes![routes::create, routes::read, routes::update, routes::delete],
        )
}
