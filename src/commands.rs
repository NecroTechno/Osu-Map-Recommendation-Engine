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
    let mut user_best_beatmaps = Vec::new();
    for score in user_best.iter() {
        user_best_beatmap_ids.push(score.beatmap_id);
    }
    for id in user_best_beatmap_ids.iter() {
        let user_best_beatmap_uri = format!(
            "https://osu.ppy.sh/api/get_beatmaps?k={}&b={}",
            env!("OSU_API_KEY"),
            id
        );
        let mut temp_beatmap_vec = Vec::new();

        let user_best_beatmap_res = reqwest::get(&user_best_beatmap_uri);

        match user_best_beatmap_res {
            Ok(response) => temp_beatmap_vec = process_beatmaps(response),
            Err(_e) => exit_with_message("Network error."),
        }
        user_best_beatmaps.push(temp_beatmap_vec[0].clone());
    }
}

pub fn confirm_user(mut res: reqwest::Response) {
    let res_content: String = res.text().unwrap();
    match res_content.as_str() {
        "[]" => exit_with_message("User not found."),
        _ => (),
    };
}
