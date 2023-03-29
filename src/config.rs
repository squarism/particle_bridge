#![allow(dead_code, unused)]
use std::fs;

use serde_json::{Result, Value};
use std::fs::File;
use std::io::BufReader;

use tera::{Context, Tera};

struct Config {
    pub config_file: String,
}

impl Config {
    pub fn new(config_file: String) -> Self {
        Self { config_file }
    }

    fn theme_definition(&self, theme: String, brightness: i32, id: String) -> Value {
        // match theme.as_str() {
        //     "bluegreen" => theme::bluegreen(brightness),
        //     "orange" => theme::orange(brightness),
        //     "blacklight" => theme::blacklight(brightness),
        //     "white" => theme::white(brightness),
        //     _ => panic!("unknown theme"),
        // }
        let mut context = Context::new();
        context.insert("brightness", &brightness);
        context.insert("id", &id);

        let config_template = fs::read_to_string(self.config_file.clone()).unwrap();
        let result = Tera::one_off(&config_template, &context, true).unwrap();

        let parsed_json: Value = serde_json::from_str(result.as_str()).unwrap();
        parsed_json["themes".to_owned()][theme].clone()
    }

    fn hosts(&self, config_file: String) -> Value {
        // make a dummy document because of the config file format
        // we need it to be populated with one section and then iterate through the other
        // but in doing so we've made a template in Tera that is unparsable until filled
        // so we have a incomplete Tera doc and broken JSON.  So render a dummy doc to
        // get the hosts, disregard this section.  Then go back to it later with the themeId
        // from the host.
        let mut context = Context::new();
        context.insert("brightness", &42);
        context.insert("id", &"dummy");

        let config_template = fs::read_to_string(config_file).unwrap();
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
        let c = Config::new("config.json.tera".to_owned());
        let result = c.config_file;
        let expected = "config.json.tera";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hosts() {
        let c = Config {
            config_file: "config.json.tera".to_owned(),
        };
        let result = c.hosts("config.json.tera".to_owned());

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
        let c = Config::new("config.json.tera".to_owned());
        let result = c.theme_definition("bluegreen".to_owned(), 42, "some-id".to_owned());

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
