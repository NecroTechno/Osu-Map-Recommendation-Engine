extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Beatmap {
    beatmapset_id: String,
    beatmap_id: String,
    approved: String,
    total_length: String,
    hit_length: String,
    version: String,
    file_md5: String,
    diff_size: String,
    diff_overall: String,
    diff_approach: String,
    diff_drain: String,
    mode: String,
    approved_date: Option<String>,
    last_update: String,
    artist: String,
    title: String,
    creator: String,
    creator_id: String,
    bpm: String,
    source: String,
    tags: String,
    genre_id: String,
    language_id: String,
    favourite_count: String,
    playcount: String,
    passcount: String,
    max_combo: String,
    difficultyrating: String,
}

fn process_beatmaps(mut res: reqwest::Response) -> Vec<Beatmap> {
    let beatmaps : Vec<Beatmap> = res.json().unwrap();
    for elem in beatmaps.iter() {
       println!("{:?}", elem);
    }
    return beatmaps;
}

fn main() {
    let request_uri = concat!(
        "https://osu.ppy.sh/api/get_beatmaps?k=",
        env!("OSU_API_KEY"),
        "&limit=1&mode=0"
    );
    let res = reqwest::get(request_uri);
    let mut beatmaps: Vec<Beatmap>;

    match res {
        Ok(response) => beatmaps = process_beatmaps(response),
        Err(e) => println!("{}", e),
    };
}
