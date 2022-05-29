mod private {
    pub const PREFIX: &str = "!";
    pub const TOKEN: &str =
        "OTc2MjI1OTg0ODYyODc5ODI0.G6YFle.YGmB0wvOvC_BgfqSNkRXOw4w75aUHPq1QKme0M";
}

use ron::{de, ser};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    token: String,
    prefix: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            token: String::from(private::TOKEN),
            prefix: String::from(private::PREFIX),
        }
    }

    pub fn token(&self) -> &str {
        self.token.as_str()
    }

    pub fn prefix(&self) -> &str {
        self.prefix.as_str()
    }

    /* Saves the configuration data into 'config.ron'. */
    pub fn save(&self) -> std::io::Result<()> {
        let data = Config {
            token: String::from(private::TOKEN),
            prefix: String::from(private::PREFIX),
        };

        let pretty = ser::PrettyConfig::new()
            .depth_limit(2)
            .separate_tuple_members(true)
            .enumerate_arrays(true);

        let s = ser::to_string_pretty(&data, pretty).expect("Serialization failed!");

        let mut file = std::fs::File::create("config.ron")?;
        if let Err(why) = write!(file, "{}", s) {
            println!("Failed writing to file: {}", why);
        } else {
            println!("Write operation succeeded!");
        }
        Ok(())
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
