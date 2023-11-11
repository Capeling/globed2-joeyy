use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    path::Path,
};

use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use serde_json::{ser::PrettyFormatter, Serializer};

/* stinky serde defaults */

fn default_web_mountpoint() -> String {
    "/".to_string()
}

fn default_web_address() -> String {
    "0.0.0.0:41000".to_string()
}

fn default_gdapi() -> String {
    "http://www.boomlings.com/database/getGJComments21.php".to_string()
}

fn default_special_users() -> HashMap<i32, SpecialUser> {
    HashMap::new()
}

fn default_secret_key() -> String {
    let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    format!("Change-Me-Please-Insecure-{}", rand_string)
}

fn default_challenge_expiry() -> u32 {
    60u32
}

fn default_challenge_level() -> i32 {
    1
}

fn default_token_expiry() -> u64 {
    60 * 30
}

// special user defaults

fn default_su_color() -> String {
    "#ffffff".to_string()
}

/* end stinky serde defaults */

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct SpecialUser {
    pub name: String,
    #[serde(default = "default_su_color")]
    pub color: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ServerConfig {
    #[serde(default = "default_web_mountpoint")]
    pub web_mountpoint: String,
    #[serde(default = "default_web_address")]
    pub web_address: String,
    #[serde(default = "default_gdapi")]
    pub gd_api: String,
    #[serde(default = "default_special_users")]
    pub special_users: HashMap<i32, SpecialUser>,

    // security
    #[serde(default = "default_secret_key")]
    pub secret_key: String,
    #[serde(default = "default_secret_key")]
    pub game_server_password: String,
    #[serde(default = "default_challenge_expiry")]
    pub challenge_expiry: u32,
    #[serde(default = "default_challenge_level")]
    pub challenge_level: i32,
    #[serde(default = "default_token_expiry")]
    pub token_expiry: u64,
}

impl ServerConfig {
    pub fn load(source: &Path) -> anyhow::Result<Self> {
        Ok(serde_json::from_reader(File::open(source)?)?)
    }

    pub fn save(&self, dest: &Path) -> anyhow::Result<()> {
        let writer = OpenOptions::new().write(true).create(true).open(dest)?;

        // i hate 2 spaces i hate 2 spaces i hate 2 spaces
        let mut serializer =
            Serializer::with_formatter(writer, PrettyFormatter::with_indent(b"    "));
        self.serialize(&mut serializer)?;

        Ok(())
    }

    pub fn reload_in_place(&mut self, source: &Path) -> anyhow::Result<()> {
        let conf = ServerConfig::load(source)?;
        self.clone_from(&conf);
        Ok(())
    }

    pub fn make_default() -> Self {
        Self {
            web_mountpoint: default_web_mountpoint(),
            web_address: default_web_address(),
            gd_api: default_gdapi(),
            special_users: default_special_users(),
            secret_key: default_secret_key(),
            game_server_password: default_secret_key(),
            challenge_expiry: default_challenge_expiry(),
            challenge_level: default_challenge_level(),
            token_expiry: default_token_expiry(),
        }
    }
}