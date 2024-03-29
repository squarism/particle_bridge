#![allow(dead_code, unused)]
use std::fs;

use serde_json::{Result, Value};
use std::fs::File;
use std::io::BufReader;

use tera::{Context, Tera};

pub struct Config {
    pub config_file: String,
}

impl Config {
    pub fn new(config_file: String) -> Self {
        Self { config_file }
    }

    pub fn theme_definition(&self, theme: String, brightness: f32, id: &str) -> Value {
        let mut context = Context::new();
        context.insert("brightness", &brightness);
        context.insert("id", &id);

        let config_template = fs::read_to_string(self.config_file.clone())
            .unwrap_or_else(|_| panic!("Config file {} not found", &self.config_file.as_str()));

        let result = Tera::one_off(&config_template, &context, true).unwrap();

        let parsed_json: Value = serde_json::from_str(result.as_str()).unwrap();
        parsed_json["themes".to_owned()][theme].clone()
    }

    pub fn hosts(&self) -> Value {
        // make a dummy document because of the config file format
        // we need it to be populated with one section and then iterate through the other
        // but in doing so we've made a template in Tera that is unparsable until filled
        // so we have a incomplete Tera doc and broken JSON.  So render a dummy doc to
        // get the hosts, disregard this section.  Then go back to it later with the themeId
        // from the host.
        let mut context = Context::new();
        context.insert("brightness", &42);
        context.insert("id", &"dummy");

        let config_template = fs::read_to_string(self.config_file.clone())
            .unwrap_or_else(|_| panic!("Config file {} not found", &self.config_file.as_str()));

        let result = Tera::one_off(&config_template, &context, true).unwrap();

        let parsed_json: Value = serde_json::from_str(result.as_str()).unwrap();
        parsed_json["pixelblazes".to_owned()].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_config_file_path() {
        let c = Config::new("fixtures/config.json.tera".to_owned());
        let result = c.config_file;
        let expected = "fixtures/config.json.tera";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hosts() {
        let c = Config::new("fixtures/config.json.tera".to_owned());
        let result = c.hosts();

        let expected = json!([
            {
                "host": "1.2.3.4",
                "themeIds": {
                    "bluegreen": "HbjL4SMNcdpCWPN2E"
                }
            }
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_theme_definition() {
        let c = Config::new("fixtures/config.json.tera".to_owned());
        let result = c.theme_definition("bluegreen".to_owned(), 0.42, "some-id");

        let expected = json!({
            "brightness": 42,
            "activeProgramId": "some-id",
                "setVars": {
                    "speed": 1.4,
                    "waveLength": 0.90,
                    "depth": 0.06
                },
                "setControls": {
                    "hsvPickerHue": [ 0.61, 0.73, 0.95 ]
                }
        });
        assert_eq!(result, expected);
    }
}
