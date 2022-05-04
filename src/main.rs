mod clipboard;
mod keyboard;
mod svg_handler;
mod config;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};

use std::{fs, thread};

use clipboard::clipboard::{get_clipboard_content, insert_clipboard_content};
use evdev::Key;
use keyboard::keyboard::KeyState;
use keyboard::keyboard::KeyboardEvent;

use svg_handler::svg_handler::SVGFigure;

use config::config::ConfigParser;

fn main() {
    let config = ConfigParser::new().expect("Please create a file under $XDG_HOME_CONFIG names inkscape-helper.toml for the configuration");
    let (send, recv): (Sender<KeyboardEvent>, Receiver<KeyboardEvent>) = channel();
    let mut kb = keyboard::keyboard::Keyboard::new(None);
    let mut key_states: HashMap<Key, KeyState> = HashMap::new();

    let _ = thread::spawn(move || {
        kb.read_event_loop(send);
    });
    loop {
        match recv.recv() {
            Ok(msg) => {
                key_states.insert(msg.get_key(), msg.get_state());
            }
            _ => {}
        }
        match key_states.get(&Key::KEY_F) {
            Some(v) => {
                if *v == KeyState::Released {
                    key_states.insert(Key::KEY_F, KeyState::Released);
                    match get_clipboard_content() {
                        Some(content) => {
                            let mut figure = SVGFigure::new(&content);
                            match figure.as_mut() {
                                Ok(f) => {
				    let style = config.config.get(&Key::KEY_F).expect("errore");
                                    f.apply_style_g_tag(*style);
                                    let updated = f.to_string();
                                    println!("inserisco",);
                                    insert_clipboard_content(&updated);
                                }
                                Err(e) => panic!("{}", e),
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        // println!("{:?}", key_states);
    }
}
