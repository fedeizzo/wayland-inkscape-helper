use std::{collections::HashMap, env, fs};

use evdev::Key;
use minidom::Element;
use toml_edit::{Document, Value};

use crate::svg_handler::styles::get_styles;

const DEFAULT_CONFIG_PATH: &str = "~/.config";

fn get_styles_from_config(config: &Document) -> HashMap<Key, fn(&mut Element) -> ()> {
    let available_styles = get_styles();
    let mut style_keymaps = HashMap::new();
    for (name, style) in available_styles.into_iter() {
	if let Some(keymap) = config["keymaps"].get(name) {
	    if let Some(Value::Integer(val)) = keymap.as_value() {
		let val = val.value();
		style_keymaps.insert(Key::new(*val as u16), style);
	    }
	}
	    
    }
    style_keymaps
}
pub struct ConfigParser {
    pub config: HashMap<Key, fn(&mut Element) -> ()>,
}

impl ConfigParser {
    fn load_config_text() -> Option<Document> {
        let parsed: Document;
        match env::var("XDG_CONFIG_HOME") {
            Ok(val) => {
                let path = val + "/inkscape-helper.toml";
                let path = shellexpand::tilde(&path).into_owned();
                let content = fs::read_to_string(path).ok()?;
                parsed = content.parse::<Document>().ok()?;
            }
            _ => {
                let path = DEFAULT_CONFIG_PATH.to_string() + "/inkscape-helper.toml";
                let path = shellexpand::tilde(&path).into_owned();
                let content = fs::read_to_string(&path).ok()?;
                parsed = content.parse::<Document>().ok()?;
            }
        }
        Some(parsed)
    }
    pub fn new() -> Option<ConfigParser> {
        let config = ConfigParser::load_config_text()?;
        Some(ConfigParser {
            config: get_styles_from_config(&config)
        })
    }
}
