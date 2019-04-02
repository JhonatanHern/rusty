extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate tokio_core;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate twitter_api;
extern crate oauth_client as oauth;

use std::time;
use std::env;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

mod config;
mod twitter;

const CONF_FILENAME: &'static str = ".crypto-bot.conf";

fn get_home_dir() -> PathBuf {
    match env::home_dir() {
        Some(p) => p,
        None => {
            panic!("Impossible to get your home dir!");
        }
    }
}

pub fn build_message () -> String {
    let mut s = String::from("THIS IS A BOT, NO NOT FEED IT");
    return s;
}

fn main() {
    let mut conf_file_path: PathBuf = get_home_dir();
    conf_file_path.push(Path::new(CONF_FILENAME));


    let config = match config::Config::read(&conf_file_path) {
        Some(v) => v,
        None => panic!("Cannot find config file"),
    };

    let twitter = twitter::Twitter::new(config.consumer_key, config.consumer_secret,
                                        config.access_key, config.access_secret);


    loop {
        let msg = build_message();
        twitter.tweet(msg);
        std::thread::sleep(time::Duration::from_millis(1000 * config.interval_sec));
    }
}
