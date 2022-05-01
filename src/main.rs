mod clipboard;
mod keyboard;
mod svg_handler;
// use std::collections::HashMap;
// use std::rc::Rc;
// use std::sync::mpsc::{channel, Receiver, Sender};
// use std::sync::Arc;
// use std::thread;

// use evdev::Key;
// use keyboard::keyboard::{KeyState, KeyboardEvent};
use clipboard::clipboard::{get_clipboard_content, insert_clipboard_content};
use std::io::Read;
use std::process::Command;
use svg_handler::styles::fill_to_grey;
use svg_handler::svg_handler::SVGFigure;

fn main() {
    match get_clipboard_content() {
        Some(content) => {
            let mut figure = SVGFigure::new(&content);
            match figure.as_mut() {
                Ok(f) => {
                    f.apply_style_g_tag(fill_to_grey);
		    let updated = f.to_string();
                    insert_clipboard_content(&updated);
                }
                Err(e) => panic!("{}", e),
            }
        }
        _ => {}
    }
    // let (send, recv): (
    //     Sender<KeyboardEvent>,
    //     Receiver<KeyboardEvent>,
    // ) = channel();
    // let mut kb = keyboard::keyboard::Keyboard::new(None);
    // let mut key_states: HashMap<Key, KeyState> = HashMap::new();

    // let _ = thread::spawn(move || {
    // 	kb.read_event_loop(send);
    // });
    // loop {
    // 	match recv.recv() {
    // 	    Ok(msg) => {key_states.insert(msg.get_key(), msg.get_state());},
    // 	    _ => {}
    // 	}
    // 	println!("{:?}", key_states);
    // }
}
