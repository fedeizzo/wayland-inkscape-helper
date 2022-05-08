use std::{borrow::BorrowMut, collections::HashMap, env, fs, rc::Rc};

use evdev::Key;
use minidom::Element;
use toml_edit::{Document, Value};

use crate::{
    keyboard::keyboard::KeyState,
    svg_handler::styles::{change_style, Style},
};

const DEFAULT_CONFIG_PATH: &str = "~/.config";

fn get_styles_from_config(config: &mut Document) -> Option<Vec<Style>> {
    let mut styles = Vec::new();
    let keymaps = config.remove("keymaps")?;
    let keymaps = keymaps.into_array_of_tables().ok()?;
    for i in keymaps.into_iter() {
        let mut style_table: HashMap<String, String> = HashMap::new();
        let inner_tbl = i.into_inline_table();
        let name = inner_tbl["name"].as_str()?;
        let keycode = inner_tbl["keycode"].as_array()?;
        let style = inner_tbl["style"].as_inline_table()?;
        for (style_name, style_value) in style.iter() {
            if let Some(val) = style_value.as_str() {
                style_table.insert(style_name.to_string(), val.to_string());
            } else if let Some(val) = style_value.as_integer() {
                style_table.insert(style_name.to_string(), val.to_string());
            } else if let Some(val) = style_value.as_float() {
                style_table.insert(style_name.to_string(), val.to_string());
            } else if let Some(val) = style_value.as_bool() {
                style_table.insert(style_name.to_string(), val.to_string());
            }
        }
        let mut key = Vec::new();
        for mut k in keycode.into_iter() {
            match k.borrow_mut() {
                Value::Integer(val) => {
                    let tmp = val.value();
                    key.push(*tmp as u16);
                }
                _ => {}
            }
        }

        styles.push(Style::new(
            name.to_string(),
            key,
            Rc::new(Box::new(move |node| change_style(&style_table, node))),
        ));
    }
    Some(styles)
}

fn get_keyboards_from_config(config: &Document) -> Option<Vec<String>> {
    let mut keyboards = Vec::new();
    if let Some(kbds) = config.get("keyboard") {
        if let Some(Value::Array(val)) = kbds["device_names"].as_value() {
            for kbd in val.iter() {
                match kbd {
                    Value::String(name) => {
                        let n = name.value();
                        keyboards.push(n.clone());
                    }
                    _ => {}
                }
            }
        }
    }
    if keyboards.len() == 0 {
        return None;
    } else {
        return Some(keyboards);
    }
}
pub struct ConfigParser {
    keymap_styles: Vec<Style>,
    pub keyboards: Option<Vec<String>>,
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
        let mut config = ConfigParser::load_config_text()?;
        let styles = get_styles_from_config(&mut config);
        Some(ConfigParser {
            keymap_styles: styles.unwrap_or(Vec::new()),
            keyboards: get_keyboards_from_config(&config),
        })
    }
    pub fn get_style(
        &self,
        last_key: &Option<Key>,
        key_states: &HashMap<Key, KeyState>,
    ) -> Option<(Rc<Box<dyn Fn(&mut Element) -> () + '_>>, Key)> {
        for (key, state) in key_states.into_iter() {
            if *state == KeyState::Pressed {
                for style in self.keymap_styles.iter() {
                    match style.key.len() {
                        1 => {
                            if style.key[0] == key.code() {
                                println!("Returning style with name {}", style.name);
                                return Some((style.style_closure.clone(), key.clone()));
                            }
                        }
                        2 => {
                            if last_key.is_some() {
                                if (style.key[0] == key.code()
                                    && style.key[1] == last_key.unwrap().code())
                                    || (style.key[1] == key.code()
                                        && style.key[0] == last_key.unwrap().code())
                                {
                                    println!("Returning style with name {}", style.name);
                                    return Some((style.style_closure.clone(), key.clone()));
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        None
    }
}
