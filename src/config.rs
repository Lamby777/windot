use std::path::Path;

use emojis::SkinTone;
use serde::{Deserialize, Serialize};

use super::*;

pub fn user_data_dir() -> PathBuf {
    dirs::data_dir().unwrap().join(APP_ID)
}

pub fn config_file_path() -> PathBuf {
    user_data_dir().join("config.json")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub preferred_skin_tone: SkinTone,
    pub recent_emojis: Vec<&'static Emoji>,
}

impl Config {
    pub fn load_or_create() -> Self {
        let config_path = config_file_path();

        if !config_path.exists() {
            let config = Self::default();
            config.save();
            config
        } else {
            Self::load(&config_path)
        }
    }

    pub fn load(path: &Path) -> Self {
        let config_file = fs::read_to_string(path).unwrap();
        serde_json::from_str(&config_file).unwrap()
    }

    pub fn save(&self) {
        let config_file = serde_json::to_string(self).unwrap();
        fs::write(config_file_path(), config_file).unwrap();
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            preferred_skin_tone: SkinTone::Default,
            recent_emojis: vec![],
        }
    }
}
