mod clipboard;
mod config;
mod keyboard;
mod svg_handler;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};

use std::thread;

use clipboard::clipboard::{get_clipboard_content, insert_clipboard_content};
use evdev::Key;
use keyboard::keyboard::KeyState;
use keyboard::keyboard::KeyboardEvent;

use svg_handler::svg_handler::SVGFigure;

use config::config::ConfigParser;

fn main() {
    let config = ConfigParser::new().expect("Please create a file under $XDG_HOME_CONFIG names inkscape-helper.toml for the configuration");
    let (send, recv): (Sender<KeyboardEvent>, Receiver<KeyboardEvent>) = channel();
    let mut kb = keyboard::keyboard::Keyboard::new(&config.keyboards);
    let mut key_states: HashMap<Key, KeyState> = HashMap::new();
    let mut last_key: Option<Key> = None;
    let mut current_key: Option<Key> = None;
    let mut first_key = true;

    let _ = thread::spawn(move || {
        kb.read_event_loop(send);
    });
    loop {
        match recv.recv() {
            Ok(msg) => {
                let key = msg.get_key();
                let state = msg.get_state();
                key_states.insert(key, state);
		if !first_key {
		    last_key = current_key;
		}
                current_key = Some(key);
		first_key = false;
            }
            _ => {}
        }
        if let Some((style_closure, key)) = config.get_style(&last_key, &key_states) {
            key_states.insert(key, KeyState::Released);
            match get_clipboard_content() {
                Some(content) => {
                    let mut figure = SVGFigure::new(&content);
                    match figure.as_mut() {
                        Ok(f) => {
                            f.apply_style_g_tag(&*style_closure);
                            let updated = f.to_string();
                            insert_clipboard_content(&updated);
                        }
                        Err(e) => eprintln!("{}", e),
                    }
                }
                _ => {}
            }
        }
    }
}
