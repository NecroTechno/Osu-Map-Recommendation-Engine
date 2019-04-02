extern crate chrono;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod structs;

use self::structs::Beatmap;

fn process_beatmaps(mut res: reqwest::Response) -> Vec<Beatmap> {
    let res_content: String = res.text().unwrap();
    let beatmaps: Vec<Beatmap> = serde_json::from_str(&res_content).unwrap();
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
