use serde::Deserialize;
use std::{collections::HashMap, fs};

pub mod socket;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[derive(Deserialize)]
pub struct Config<'a> {
    #[serde(borrow)]
    pub base_url: &'a str,
    #[serde(borrow)]
    pub nbk: &'a str,
    #[serde(borrow)]
    pub rooms: HashMap<&'a str, HashMap<&'a str, u8>>,
}

#[derive(Clone, Debug)]
pub struct BotData {
    pub team: u8,
    pub room: &'static str,
}

lazy_static! {
    static ref CONFIG_STRING: String = fs::read_to_string("config.toml").unwrap();
    pub static ref CONFIG: Config<'static> = toml::from_str(&CONFIG_STRING).unwrap();
    pub static ref BOT_DATA: Vec<BotData> = {
        let mut ans = Vec::new();

        for (room, teams) in &CONFIG.rooms {
            for (team, &num) in teams {
                for _ in 0..num {
                    ans.push(BotData {
                        room,
                        team: team.parse().unwrap(),
                    });
                }
            }
        }

        ans
    };
}
