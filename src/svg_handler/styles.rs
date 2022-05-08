use std::{collections::HashMap, rc::Rc};

use minidom::Element;

fn decode_style(style: &str) -> HashMap<&str, &str> {
    let mut decoded: HashMap<&str, &str> = HashMap::new();
    for s in style.split(";").into_iter() {
        let value: Vec<&str> = s.split(":").collect();
        decoded.insert(value[0], value[1]);
    }
    return decoded;
}

fn encode_style(style: &HashMap<&str, &str>) -> String {
    let mut encoded = String::new();
    for (k, v) in style.into_iter() {
        encoded.push_str(&format!("{}:{};", k, v));
    }
    return encoded;
}

pub struct Style {
    pub name: String,
    pub key: Vec<u16>,
    pub style_closure: Rc<Box<dyn Fn(&mut Element) -> ()>>,
}

impl Style {
    pub fn new(name: String, key: Vec<u16>, style: Rc<Box<dyn Fn(&mut Element) -> ()>>) -> Style {
        Style { name, key, style_closure: style }
    }
}

pub fn change_style(style: &HashMap<String, String>, node: &mut Element) {
    match node.attr("style") {
        Some(s) => {
            let mut new_style = decode_style(s);
	    println!("here", );
            for (k, v) in style.iter() {
                new_style.insert(&k, &v);
            }
            node.set_attr("style", encode_style(&new_style));
        }
        _ => {}
    }
}
