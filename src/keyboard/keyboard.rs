use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap,
    sync::{mpsc::Sender, Arc},
};

use evdev::{Device, EventType, Key};

fn load_keyboard() -> Option<Device> {
    for input_number in 0..27 {
        let device = format!("/dev/input/event{}", input_number);
        let keyboard = Device::open(device).unwrap();
        let supported = keyboard
            .supported_keys()
            .map_or(false, |keys| keys.contains(Key::KEY_ENTER));
        if supported {
            return Some(keyboard);
        }
    }
    None
}

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
    pub fn new(device_number: Option<i32>) -> Keyboard {
        match device_number {
            Some(num) => Keyboard {
                device: Device::open(format!("/device/event{}", num))
                    .expect("Event number passed is not a valid device"),
		event: None
            },
            _ => Keyboard {
                device: load_keyboard().expect("No valid keyboard found"),
		event: None
            },
        }
    }

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
			    self.event = Some(KeyboardEvent{
				key:Key::new(key),state: KeyState::Holded
			    });
                        } else if event.value() == 0 {
			    self.event = Some(KeyboardEvent{
				key:Key::new(key),state: KeyState::Released
			    });
                        } else {
			    self.event = Some(KeyboardEvent{
				key:Key::new(key),state: KeyState::Pressed
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