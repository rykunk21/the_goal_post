#[macro_use]
extern crate rocket;
use std::net::{IpAddr, Ipv4Addr};

use rocket::{
    fs::{relative, FileServer},
    Config,
};

mod routes;
use routes::DatabaseFairing;

mod db;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(DatabaseFairing)
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
            routes![
                // Team routes
                routes::create_team,
                routes::get_team,
                routes::get_all_teams,
                routes::update_team,
                routes::delete_team,
                // Game routes
                routes::create_game,
                routes::get_game,
                routes::get_all_games,
                routes::get_games_by_week,
                routes::update_game,
                routes::delete_game,
                // Betting line routes
                routes::create_betting_line,
                routes::get_betting_line,
                routes::get_betting_lines_for_game,
                // Prediction routes
                routes::create_prediction,
                routes::get_prediction,
                routes::get_prediction_for_game,
            ],
        )
}
