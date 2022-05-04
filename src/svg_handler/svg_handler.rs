use std::collections::VecDeque;
use std::str::from_utf8;

use minidom::{quick_xml::Reader, Element};

pub struct SVGFigure {
    content: Element,
}

impl ToString for SVGFigure {
    fn to_string(&self) -> String {
        let mut buf = Vec::new();
        let _ = self.content.write_to(&mut buf);
        match from_utf8(&buf) {
            Ok(extracted) => extracted.to_string(),
            _ => "".to_string(),
        }
    }
}

impl SVGFigure {
    pub fn new(svg_string: &str) -> Result<SVGFigure, &'static str> {
        let content = Element::from_reader(&mut Reader::from_str(svg_string));
        match content {
            Ok(tree) => Ok(SVGFigure { content: tree }),
            // _ => Err("Unable to load svg from the provided string"),
            Err(e) => {
                println!("{:?}", e);
                Err("Unable to load svg from the provided string")
            }
        }
    }

    pub fn apply_style_g_tag(&mut self, style: fn(&mut Element) -> ()) {
        let mut queue: VecDeque<&mut Element> = VecDeque::from([&mut self.content]);
        while !queue.is_empty() {
            let current_node = queue.pop_front().unwrap();
            if current_node.name() == "g" {
                for child in current_node.children_mut() {
                    style(child);
                }
            }
            for child in current_node.children_mut() {
                queue.push_back(child);
            }
        }
    }
}
