use config::{Config, Environment, File, FileFormat};
use errors::*;

#[derive(Debug, Default, Deserialize)]
pub struct Settings {
    pub host: Option<String>,
    pub apikey: Option<String>,
}
impl Settings {
    pub fn new() -> Result<Self> {
        let mut s = Config::new();

        // read config file in local directory
        s.merge(File::with_name(".rrconfig")
                .format(FileFormat::Yaml)
                .required(false))?;

        // add in settings from environment (with prefix "RR")
        s.merge(Environment::with_prefix("rr"))?;

        s.try_into().chain_err(|| "Can't create settings from configuration")
    }
}
