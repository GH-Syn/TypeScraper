#!ignore(unused_variables)
use colored::*;
use serde::Deserialize;
use serde_json::Result;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub rank: u16,
    pub racer: String,
    pub text_bests: f64,
    pub races: u16,
    pub texts: u16,
    pub career: f64,
    pub best_10: f64,
    pub best_race: f64,
    pub points: f64,
    pub wins: u16,
    pub win_ratio: u8,
    pub marathon: u16,
    pub last_race: String,
    pub variation: f64
}

pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for Profile {
    fn summarize(&self) -> String {
        format!(" 🏎️ {}\n 📅 {}\n 🏆 {}",
                self.races.to_string().red().bold(),
                self.last_race.to_string().white().bold(),
                self.win_ratio.to_string().yellow().bold())
    }
}

fn print_profiles(profiles: &Vec<Profile>) {
    /* Prints profiles with all of it's summarized properties */
    for profile in profiles {
        println!("{}", profile.summarize());
    }
}


fn load_data(file: &str) -> File {
    let file = File::open(file).expect("File not found");
    return file;
}

fn main() -> Result<()> {
    // Create an instance of a profile
    let _test = load_data("profiles.json");

    // TODO fill in with json values
    let profile = Profile {
        rank: 2,
        racer: String::from("test").to_string(),
        text_bests: 2.3,
        races: 23,
        texts: 231,
        career: 21.3,
        best_10: 22.1,
        best_race: 22.1,
        points: 22.3,
        wins: 12,
        win_ratio: 2,
        marathon: 201,
        last_race: String::from("test"),
        variation: 22.0
    };

    // Add profile to profiles
    let profiles:Vec<Profile> = Vec::from([profile]);
    print_profiles(&profiles);
    Ok(())
}

