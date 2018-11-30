
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

use crate::models::BotConfig;

use magic_crypt::MagicCrypt;

use log::*;
use serde::Serialize;
use serde::de::DeserializeOwned;

extern crate dirs;

pub fn read_file(filepath: &str) -> String {
    let content_read = match File::open(filepath) {
        Err(_why) => {
            error!("could not read {}", filepath);
            String::new()
        },
        Ok(file) => {
            let mut buffered_reader = BufReader::new(file);
            let mut contents = String::new();
            let _number_of_bytes: usize = match buffered_reader.read_to_string(&mut contents) {
                Ok(number_of_bytes) => number_of_bytes,
                Err(_err) => 0
            };
            info!("read file {} successfully", filepath);
            contents
        }
    };
    content_read
}

pub fn write_file(content: &str, filepath: &str) {
    let path = Path::new(filepath);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why),
        Ok(file) => file,
    };

    match file.write_all(content.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                   why)
        },
        Ok(_) => info!("successfully wrote to {}", display),
    }
}

/// assembles a file path to the filter rules
/// location
pub fn assemble_bot_filepath(actual_file: &str) -> String {
    let mut target = dirs::config_dir().unwrap();
    target.push("twitterbot-rs");
    target.push(actual_file);
    target.into_os_string().into_string().unwrap()
}

pub fn get_json_from_storage<T>(name: &str) -> Option<T> where T: DeserializeOwned {
    let file_content: String = read_file(assemble_bot_filepath(name).as_str());
    serde_json::from_str::<T>(&file_content).ok()
}

pub fn persist_json_to_storage<T>(filters: T, name: &str) where T: Serialize {
    let content_str = serde_json::to_string_pretty::<T>(&filters).unwrap();
    write_file(content_str.as_str(), &assemble_bot_filepath(name).as_str());
}

pub fn get_config_from_storage() -> BotConfig {
    let base64: String = read_file(assemble_bot_filepath("config.json").as_str());
    let  master_pwd = read_file(&assemble_bot_filepath("master.dat"));
    let mut mc: MagicCrypt = new_magic_crypt!(master_pwd, 256);
    let file_content = mc.decrypt_base64_to_string(&base64).unwrap();
    match serde_json::from_str(&file_content) {
        Ok(persisted_config) => persisted_config,
        Err(_err) => BotConfig::default()
    }
}

pub fn persist_config_to_storage(config: BotConfig) {
    let content_str = serde_json::to_string_pretty(&config).unwrap();
    let  master_pwd = read_file(&assemble_bot_filepath("master.dat"));
    let mut mc: MagicCrypt = new_magic_crypt!(master_pwd, 256);
    let base64 = mc.encrypt_str_to_base64(content_str);
    write_file(base64.as_str(), &assemble_bot_filepath("config.json").as_str());
}

