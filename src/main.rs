#!ignore(unused_variables)
use colored::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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
        format!(" 🏎️ {},\n 📅 {}",
        self.races.to_string().red().bold(),
        self.last_race.to_string().white().bold())
    }
}

fn print_profiles(profiles: &Vec<Profile>) {
    /* Prints profiles with all of it's summarized properties */
    for profile in profiles {
        println!("{}", profile.summarize());
    }
}

fn load_data() -> Profile{
    let mut file = File::open("data.json")
        .expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong with reading the file");

    let profile_data: Profile = serde_json::from_str(&contents).unwrap();
    return profile_data;
}

fn main() {
    // Create an instance of a profile
    let profile = Profile {
        username: String::from("Joshua Rose"),
        content: String::from("Donkeys are pretty cool!"),
        replies: Vec::new(),
        retweets: 3,
        likes: 39
    };

    // Add profile to profiles
    let profiles:Vec<Profile> = Vec::from([profile]);
    print_profiles(&profiles)
}
