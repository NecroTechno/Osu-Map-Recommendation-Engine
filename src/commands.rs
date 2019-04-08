use super::structs::Beatmap;
use super::structs::UserBestScore;
use std::process;

use rand::seq::SliceRandom;

pub fn exit_with_message(message: &str) {
    println!("{}", message);
    process::exit(1);
}

fn average(numbers: &[f32]) -> f32 {
    numbers.iter().sum::<f32>() as f32 / numbers.len() as f32
}

pub fn process_beatmaps(mut res: reqwest::Response) -> Vec<Beatmap> {
    let res_content: String = res.text().unwrap();
    let beatmaps: Vec<Beatmap> = serde_json::from_str(&res_content).unwrap();
    return beatmaps;
}

pub fn process_user_best(mut res: reqwest::Response) -> f32 {
    println!("");
    println!("Calculating...");
    let res_content: String = res.text().unwrap();
    let user_best: Vec<UserBestScore> = serde_json::from_str(&res_content).unwrap();
    let mut user_best_beatmap_ids = Vec::new();
    let mut user_best_beatmap_difficulty = Vec::new();
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
        user_best_beatmap_difficulty.push(temp_beatmap_vec[0].difficultyrating);
    }
    let average_difficulty_of_best = average(&user_best_beatmap_difficulty);
    println!("");
    println!(
        "The average difficulty of your 10 best maps is: {}",
        average_difficulty_of_best
    );
    println!("");
    return average_difficulty_of_best;
}

pub fn recommend_maps(average_difficulty_of_best: f32, possible_beatmaps: Vec<Beatmap>) {
    let mut acceptable_beatmaps = Vec::new();
    for beatmap in possible_beatmaps.iter() {
        if beatmap.difficultyrating < average_difficulty_of_best + 0.5 {
            if beatmap.difficultyrating >= average_difficulty_of_best {
                acceptable_beatmaps.push(beatmap.clone());
            }
        }
    }
    if acceptable_beatmaps.is_empty() {
        exit_with_message("The oracle found no acceptable beatmaps. Try again.")
    }

    let recommendations: Vec<_> = acceptable_beatmaps
        .choose_multiple(&mut rand::thread_rng(), 5)
        .collect();
    println!("");
    println!("Here are some maps that might work for you.");
    println!("");
    for (i, beatmap) in recommendations.iter().enumerate() {
        println!("Try {}", beatmap.title);
        println!("This map is available at:");
        println!("https://osu.ppy.sh/beatmaps/{}", beatmap.beatmap_id);
        if i + 1 != recommendations.len() {
            println!("~~~");
        }
    }
}

pub fn confirm_user(mut res: reqwest::Response) {
    let res_content: String = res.text().unwrap();
    match res_content.as_str() {
        "[]" => exit_with_message("User not found."),
        _ => (),
    };
}
