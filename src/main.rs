extern crate chrono;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate text_io;

mod commands;
mod structs;

use self::commands::{confirm_user, exit_with_message, process_beatmaps, process_user_best};

/*
let request_uri = concat!(
    "https://osu.ppy.sh/api/get_beatmaps?k=",
    env!("OSU_API_KEY"),
    "&limit=1&mode=0"
);
*/

fn main() {
    println!("Please enter your username.");
    let username: String = read!();
    let user_request_uri = format!(
        "https://osu.ppy.sh/api/get_user?k={}&u={}",
        env!("OSU_API_KEY"),
        &username
    );

    let user_res = reqwest::get(&user_request_uri);
    // let mut beatmaps: Vec<Beatmap>;

    match user_res {
        Ok(response) => confirm_user(response),
        Err(e) => exit_with_message("Network error."),
    };

    let user_best_uri = format!(
        "https://osu.ppy.sh/api/get_user_best?k={}&u={}",
        env!("OSU_API_KEY"),
        &username
    );

    let user_best_res = reqwest::get(&user_best_uri);

    match user_best_res {
        Ok(response) => process_user_best(response),
        Err(e) => exit_with_message("Network error."),
    }
}
