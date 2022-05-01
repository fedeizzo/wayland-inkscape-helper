use std::collections::HashMap;

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
	encoded.push_str(&format!("{}:{};", k, v ));
    }
    return encoded;
}

pub fn fill_to_grey(node: &mut Element) {
    match node.attr("style") {
	Some(style) => {
	    let mut new_style = decode_style(style);
	    new_style.insert("fill", "#ff0000");
	    new_style.insert("fill-opacity", "0.12");
	    node.set_attr("style", encode_style(&new_style));
	},
	_ => {}
    }
}
