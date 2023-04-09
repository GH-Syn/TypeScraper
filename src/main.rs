use colored::*;
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, Result};
use std::fs::File;
use std::io::{stdout, Write};
use std::{collections::BTreeMap, io};

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

const DO_SUMMARY_EMOJIS: bool = true;

trait Summary {
    fn summarize(&self, profile: &Profile) -> String;
}

impl Summary for Profile {
    fn summarize(&self, profile: &Profile) -> String {
        if DO_SUMMARY_EMOJIS {
            format!(" 👨 Profile: {}\n 🏎️  Races: {}\n 📅 Latest: {}\n\n 🏆 Win: {}%\n 🎯 Best: {}\n 🏢 Career: {}\n 🐎 Marathon: {} races",
                    profile.racer.yellow().bold(),
                    profile.races.red().bold(),
                    profile.last_race.white().bold(),
                    profile.win_ratio.yellow().bold(),
                    profile.best_race.red().bold(),
                    profile.career.white().bold(),
                    profile.marathon.white().bold(),
                    )
        } else {
            format!(" Profile: {}\n Races: {}\n Latest: {}\n\n Win: {}%\n Best: {}\n Career: {}\n Marathon: {} races",
                    profile.racer.yellow().bold(),
                    profile.races.red().bold(),
                    profile.last_race.white().bold(),
                    profile.win_ratio.yellow().bold(),
                    profile.best_race.red().bold(),
                    profile.career.white().bold(),
                    profile.marathon.white().bold(),
                    )
        }
    }
}

// Datatype structure for ProfileList
type ProfileList = std::collections::BTreeMap<String, Profile>;

fn prompt_refresh(name: String) {
    print!(
        "Sorry, {} doesn\'t seem to be in the database. Would you like to refresh it?",
        &name
    );
}

/// "And now, my time is near..." - Frank Sinatra
fn say_bye() {
    println!("\n👋 Bye!\n");
    std::process::exit(1);
}

/// clear terminal screen
fn wipe() {
    stdout().flush().unwrap();
    print!("\x1B[2J\x1B[1;1H");
}

fn main() -> Result<()> {
    let file_path: &str = "src/profiles.json";
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);
    let mut profile_map: BTreeMap<String, Profile> = BTreeMap::new();
    let profile_dict: ProfileList = from_reader(reader).unwrap();

    for (_, profile) in profile_dict {
        profile_map.insert(profile.name.to_string(), profile);
    }

    loop {
        println!("Please enter the name or ID of a racer.");
        println!("Alternatively, enter 'q' to quit.\n");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let name = input.trim();

        match name {
            "q" => {
                say_bye();
                wipe();
            },
            _ => continue
        };

        if profile_map.len() > 0 {
            for (key, profile) in profile_map.iter() {
                if profile.racer.contains(&name) {
                    profile_map
                        .get(key)
                        .expect("Profile not eligable for index.")
                        .summarize(profile);
                }
            }
        }
        prompt_refresh(name.to_string());
    }
}
