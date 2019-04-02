use chrono::{DateTime, Utc, TimeZone};
use std::fmt::Display;
use std::str::FromStr;

use serde::de::{self, Deserialize, Deserializer};

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

fn from_str_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Utc.datetime_from_str(&s, FORMAT)
        .map_err(serde::de::Error::custom)
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Beatmap {
    #[serde(deserialize_with = "from_str")]
    beatmapset_id: u32,
    #[serde(deserialize_with = "from_str")]
    beatmap_id: u32,
    #[serde(deserialize_with = "from_str")]
    approved: i8,
    #[serde(deserialize_with = "from_str")]
    total_length: u16,
    #[serde(deserialize_with = "from_str")]
    hit_length: u16,
    version: String,
    file_md5: String,
    #[serde(deserialize_with = "from_str")]
    diff_size: u8,
    #[serde(deserialize_with = "from_str")]
    diff_overall: u8,
    #[serde(deserialize_with = "from_str")]
    diff_approach: u8,
    #[serde(deserialize_with = "from_str")]
    diff_drain: u8,
    #[serde(deserialize_with = "from_str")]
    mode: u8,
    approved_date: Option<String>,
    #[serde(deserialize_with = "from_str_date")]
    last_update: DateTime<Utc>,
    artist: String,
    title: String,
    creator: String,
    #[serde(deserialize_with = "from_str")]
    creator_id: u32,
    #[serde(deserialize_with = "from_str")]
    bpm: f32,
    source: String,
    tags: String,
    #[serde(deserialize_with = "from_str")]
    genre_id: u16,
    #[serde(deserialize_with = "from_str")]
    language_id: u16,
    #[serde(deserialize_with = "from_str")]
    favourite_count: u32,
    #[serde(deserialize_with = "from_str")]
    playcount: u64,
    #[serde(deserialize_with = "from_str")]
    passcount: u64,
    #[serde(deserialize_with = "from_str")]
    max_combo: u32,
    #[serde(deserialize_with = "from_str")]
    difficultyrating: f32,
}
