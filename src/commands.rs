use super::structs::Beatmap;
use super::structs::UserBestScore;
use std::process;

pub fn exit_with_message(message: &str) {
    println!("{}", message);
    process::exit(1);
}

pub fn process_beatmaps(mut res: reqwest::Response) -> Vec<Beatmap> {
    let res_content: String = res.text().unwrap();
    let beatmaps: Vec<Beatmap> = serde_json::from_str(&res_content).unwrap();
    for elem in beatmaps.iter() {
        println!("{:?}", elem);
    }
    return beatmaps;
}

pub fn process_user_best(mut res: reqwest::Response) {
    let res_content: String = res.text().unwrap();
    let user_best: Vec<UserBestScore> = serde_json::from_str(&res_content).unwrap();
    let mut user_best_beatmap_ids = Vec::new();
    for score in user_best.iter() {
        user_best_beatmap_ids.push(score.beatmap_id);
    }
    for id in user_best_beatmap_ids.iter() {
        println!("{}", id);
    }
}

pub fn confirm_user(mut res: reqwest::Response) {
    let res_content: String = res.text().unwrap();
    match res_content.as_str() {
        "[]" => exit_with_message("User not found."),
        _ => (),
    };
}
