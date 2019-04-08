extern crate chrono;
extern crate rand;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate text_io;

mod commands;
mod structs;

use self::commands::{
    confirm_user, exit_with_message, process_beatmaps, process_user_best, recommend_maps,
};
use std::io::{self, Write};
use chrono::{Duration, Utc};
use rand::Rng;

fn main() {
    let mut average_difficulty_of_best: f32 = 0.0;
    let mut possible_beatmaps = Vec::new();

    print!("Please enter your username: ");

    io::stdout().flush().unwrap();

    let mut rng = rand::thread_rng();
    let dt = Utc::now() - Duration::weeks(rng.gen_range(8, 52));

    let username: String = read!();
    let user_request_uri = format!(
        "https://osu.ppy.sh/api/get_user?k={}&u={}",
        env!("OSU_API_KEY"),
        &username
    );

    let user_res = reqwest::get(&user_request_uri);

    match user_res {
        Ok(response) => confirm_user(response),
        Err(_e) => exit_with_message("Network error."),
    };

    let user_best_uri = format!(
        "https://osu.ppy.sh/api/get_user_best?k={}&u={}",
        env!("OSU_API_KEY"),
        &username
    );

    let user_best_res = reqwest::get(&user_best_uri);

    match user_best_res {
        Ok(response) => average_difficulty_of_best = process_user_best(response),
        Err(_e) => exit_with_message("Network error."),
    }

    println!("Consulting the oracle for your recommendation...");

    let possible_maps_uri = format!(
        "https://osu.ppy.sh/api/get_beatmaps?k={}&m=0&since={}",
        env!("OSU_API_KEY"),
        dt.format("%Y-%m-%d")
    );

    let possible_maps_res = reqwest::get(&possible_maps_uri);

    match possible_maps_res {
        Ok(response) => possible_beatmaps = process_beatmaps(response),
        Err(_e) => exit_with_message("Network error."),
    }

    recommend_maps(average_difficulty_of_best, possible_beatmaps);

    println!("");
    println!("See you next time.");
}
