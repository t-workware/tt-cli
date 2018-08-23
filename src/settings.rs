use std::env;
use config::{Config, ConfigError, Environment, File};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub journal_file: String,
    pub print: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            journal_file: "journal.txt".to_string(),
            print: true,
        }
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config_file_name = env::var("TT_CLI_CONFIG_FILE_NAME")
            .unwrap_or_else(|_|"tt-cli.toml".to_string());

        let mut config = Config::new();

        config.merge(Config::try_from(&Settings::default())?)?;

        if let Ok(home) = env::var("TT_CLI_HOME") {
            config.merge(
                File::with_name(&format!("{}/{}", home, config_file_name))
                    .required(false)
            )?;
        }

        config.merge(File::with_name(&config_file_name).required(false))?;

        // Add in settings from the environment (with a prefix of TT)
        // Eg.. `TT_SET_DEBUG=1 ./target/tt-cli` would set the `debug` key
        config.merge(Environment::with_prefix("TT_SET"))?;

        let settings = config.try_into()?;
        Ok(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_default() {
        let settings = Settings::new().unwrap();
        assert_eq!("journal.txt", &settings.journal_file);
    }
}