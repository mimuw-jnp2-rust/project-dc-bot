
use ron::{de};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    token: String,
    prefix: String,
}

impl Config {
    pub fn token(&self) -> &str {
        self.token.as_str()
    }

    pub fn prefix(&self) -> &str {
        self.prefix.as_str()
    }

    /* Deserializes the configuration data from 'config.ron' and initializes app's settings. */
    pub fn load() -> std::io::Result<Config> {
        let input_path = format!("{}/config.ron", env!("CARGO_MANIFEST_DIR"));
        let f = std::fs::File::open(&input_path).expect("Failed opening file");
        let config: Config = match de::from_reader(&f) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load config: {}", e);
                std::process::exit(1);
            }
        };

        Ok(config)
    }
}
