mod private {
    pub const PREFIX: &'static str = "!";
    pub const TOKEN: &'static str = "OTc2MjI1OTg0ODYyODc5ODI0.G6YFle.YGmB0wvOvC_BgfqSNkRXOw4w75aUHPq1QKme0M";
}

use ron::{ser, de};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    token: &'static str,
    prefix: &'static str,
}

impl Config {
    pub fn new() -> Self {
        return Config {
            token: private::TOKEN,
            prefix: private::PREFIX,
        };
    }

    pub fn token(&self) -> &'static str {
        return self.token;
    }

    pub fn prefix(&self) -> &'static str {
        return self.prefix;
    }

    /* Saves the configuration data into 'config.ron'. */
    pub fn save(&self) -> std::io::Result<()> {
        let data = Config {
            token: private::TOKEN,
            prefix: private::PREFIX,
        };

        let pretty = ser::PrettyConfig::new()
            .with_depth_limit(2)
            .with_separate_tuple_members(true)
            .with_enum_arrays(true);

        let s = ser::to_string_pretty(&data, pretty)
            .expect("Serialization failed!");

        let mut file = std::fs::File::create("config.ron")?;
        if let Err(why) = write!(file, "{}", s) {
            println!("Failed writing to file: {}", why);
        } else {
            println!("Write operation succeeded!");
        }
        return Ok(());
    }

    /* Deserializes the configuration data from 'config.ron' and initializes app's settings. */
    pub fn load() -> std::io::Result<Config> {
        let input_path = format!("{}/config.ron", env!("CARGO_MANIFEST_DIR"));
        let f = std::fs::File::open(&input_path).expect("Failed opening file");
        let config: Config = match de::from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load config: {}", e);
                std::process::exit(1);
            }
        };

        return Ok(config);
    }
}