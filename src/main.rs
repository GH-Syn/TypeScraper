#!ignore(unused_variables)
use colored::*;
use serde_json::{Result, from_reader};
use std::io::BufReader;
use std::fs::File;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Profile {
    rank: String,
    racer: String,
    text_bests: String,
    races: String,
    texts: String,
    career: String,
    best_10: String,
    best_race: String,
    points: String,
    wins: String,
    win_ratio: String,
    marathon: String,
    last_race: String,
    name: String,
}

trait Summary {
    fn summarize(&self, profile: &Profile) -> String;
}

impl Summary for Profile {
    fn summarize(&self, profile: &Profile) -> String {
        format!(" 🏎️  Races: {}\n\n 📅 Latest: {}\n\n 🏆 Win: {}%",
                profile.races.red().bold(),
                profile.last_race.white().bold(),
                profile.win_ratio.yellow().bold())
    }
}

type ProfileList = std::collections::BTreeMap<String, Profile>;

fn main() -> Result<()> {
    let file = File::open("src/profiles.json").unwrap();
    let reader = BufReader::new(file);
    let profile_list: ProfileList = from_reader(reader).unwrap();

    for (_, profile) in profile_list {
        println!("{}", profile.summarize(&profile));
    }
    Ok(())
}

