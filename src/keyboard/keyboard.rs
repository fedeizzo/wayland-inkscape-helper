use std::{collections::HashMap, fs::read_dir, sync::mpsc::Sender};

use evdev::{Device, Key};

fn load_keyboard(names: &Option<Vec<String>>) -> Option<Device> {
    let mut keyboards: HashMap<String, Device> = HashMap::new();
    let devices = read_dir("/dev/input").ok()?;
    for input in devices {
        if let Ok(path) = input {
            let path = path.path().into_os_string().into_string().unwrap();
            if path.contains("event") {
                let device = Device::open(path).ok()?;
                let device_name = device.name();
                let supported = device
                    .supported_keys()
                    .map_or(false, |keys| keys.contains(Key::KEY_ENTER));
                if supported && device_name.is_some() {
                    keyboards.insert(device_name.unwrap().to_string(), device);
                }
            }
        }
    }
    match names {
        Some(ns) => {
            for n in ns.into_iter() {
                if let Some(dev) = keyboards.remove(n) {
                    return Some(dev);
                }
            }
        }
        None => {}
    }
    println!("{:?}", keyboards.keys());
    None
}

// #[derive(Debug, PartialEq, Clone)]
// pub enum ClipboradOperation {
//     Copy,
//     Paste,
// }

#[derive(Debug, PartialEq, Clone)]
pub enum KeyState {
    Pressed,
    Released,
    Holded,
}

#[derive(Debug, Clone)]
pub struct KeyboardEvent {
    key: Key,
    state: KeyState,
}

impl KeyboardEvent {
    pub fn get_state(&self) -> KeyState {
        self.state.clone()
    }
    pub fn get_key(&self) -> Key {
        self.key.clone()
    }
}

pub struct Keyboard {
    device: Device,
    event: Option<KeyboardEvent>,
}

impl Keyboard {
    pub fn new(device_names: &Option<Vec<String>>) -> Keyboard {
        Keyboard {
            device: load_keyboard(&device_names).expect("No valid keyboard found"),
            event: None,
        }
    }

    // pub fn operate_on_clipboard(&mut self, operation: ClipboradOperation) {
    //     let mut keys_to_press = [Key::KEY_CAPSLOCK, Key::KEY_V];
    //     if operation == ClipboradOperation::Copy {
    //         keys_to_press = [Key::KEY_CAPSLOCK, Key::KEY_C];
    //     }
    //     // let keys_to_press = [Key::KEY_C, Key::KEY_I, Key::KEY_A, Key::KEY_O];
    //     for key in keys_to_press.into_iter() {
    //         let result = self
    //             .device
    //             .send_events(&[InputEvent::new(EventType::KEY, key.code(), 1)]);
    //         match result {
    //             Err(_) => {
    //                 let _ = Notification::new()
    //                     .summary("Inkscape helper")
    //                     .body("Error during movement from svg content to clipboard")
    //                     .timeout(Timeout::Milliseconds(5000))
    //                     .show();
    //             }
    //             _ => {}
    //         }
    //         if key == Key::KEY_CAPSLOCK {
    //             let result =
    //                 self.device
    //                     .send_events(&[InputEvent::new(EventType::KEY, key.code(), 2)]);
    //             match result {
    //                 Err(_) => {
    //                     let _ = Notification::new()
    //                         .summary("Inkscape helper")
    //                         .body("Error during movement from svg content to clipboard")
    //                         .timeout(Timeout::Milliseconds(5000))
    //                         .show();
    //                 }
    //                 _ => {}
    //             }
    //         }
    //     }
    //     for key in keys_to_press.into_iter().rev() {
    //         let result = self
    //             .device
    //             .send_events(&[InputEvent::new(EventType::KEY, key.code(), 0)]);
    //         match result {
    //             Err(_) => {
    //                 let _ = Notification::new()
    //                     .summary("Inkscape helper")
    //                     .body("Error during movement from svg content to clipboard")
    //                     .timeout(Timeout::Milliseconds(5000))
    //                     .show();
    //             }
    //             _ => {
    //                 println!("done");
    //             }
    //         }
    //     }
    // }

    pub fn read_event_loop(&mut self, send: Sender<KeyboardEvent>) {
        let mut key: u16 = 0;
        let mut last_key: u16 = 0;
        loop {
            let mut event_type = 0;
            let events = self.device.fetch_events().unwrap();
            for event in events {
                match event_type {
                    0 => {
                        key = event.value() as u16;
                    }
                    1 => {
                        if last_key == key && event.value() == 2 {
                            self.event = Some(KeyboardEvent {
                                key: Key::new(key),
                                state: KeyState::Holded,
                            });
                        } else if event.value() == 0 {
                            self.event = Some(KeyboardEvent {
                                key: Key::new(key),
                                state: KeyState::Released,
                            });
                        } else {
                            self.event = Some(KeyboardEvent {
                                key: Key::new(key),
                                state: KeyState::Pressed,
                            });
                        }
                    }
                    2 => {
                        event_type = 0;
                        last_key = key;
                        match self.event.as_ref() {
                            Some(ev) => send.send(ev.clone()).err(),
                            _ => None,
                        };
                    }
                    _ => {}
                }
                event_type += 1;
            }
        }
    }
}
