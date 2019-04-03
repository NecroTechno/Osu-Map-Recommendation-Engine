extern crate chrono;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate text_io;

mod structs;

use std::process;
use self::structs::Beatmap;

fn exit_with_message(message: &str) {
    println!("{}", message);
    process::exit(1);
}

fn process_beatmaps(mut res: reqwest::Response) -> Vec<Beatmap> {
    let res_content: String = res.text().unwrap();
    let beatmaps: Vec<Beatmap> = serde_json::from_str(&res_content).unwrap();
    for elem in beatmaps.iter() {
        println!("{:?}", elem);
    }
    return beatmaps;
}

fn confirm_user(mut res: reqwest::Response) {
    let res_content: String = res.text().unwrap();
    let empty_match: String;
    match res_content.as_str() {
        "[]" => exit_with_message("User not found."),
        _ => (),
    };
}

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
    let request_uri = format!(
        "https://osu.ppy.sh/api/get_user?k={}&u={}",
        env!("OSU_API_KEY"),
        &username
    );

    let res = reqwest::get(&request_uri);
    // let mut beatmaps: Vec<Beatmap>;

    match res {
        Ok(response) => confirm_user(response),
        Err(e) => exit_with_message("Network error."),
    };
}
