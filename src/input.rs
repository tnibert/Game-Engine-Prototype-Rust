use crate::observer::{Observable, Observer};
use crate::piston_window::PressEvent;
use crate::piston_window::ReleaseEvent;
use std::collections::BTreeSet;
use std::cell::RefCell;

pub struct Input <'a> {
    signals_out: Observable <'a>,
    keys_down: BTreeSet<piston_window::Key>
}

impl <'a> Input <'a> {
    pub fn new() -> Input <'a> {
        Self {
            signals_out: Observable::new("input".to_string()),
            keys_down: BTreeSet::new()
        }
    }

    pub fn subscribe(&mut self, subscriber: &'a RefCell<dyn Observer>, event_names: Vec<&str>) {
        for en in event_names {
            self.signals_out.subscribe(en.to_string(), &subscriber);
        }
    }

    // todo: what is a good name for this function?
    pub fn handle_event(&mut self, e: &piston_window::Event) {
        // key pressed down
        if let Some(piston_window::Button::Keyboard(k)) = e.press_args() {
            self.keys_down.insert(k);
        }

        // key released
        if let Some(piston_window::Button::Keyboard(k)) = e.release_args() {
            self.keys_down.remove(&k);
        }

        for k in &self.keys_down {
            match k {
                piston_window::Key::Right => {
                    self.signals_out.notify("right".to_string());
                },
                piston_window::Key::Left => {
                    self.signals_out.notify("left".to_string());
                },
                piston_window::Key::Down => {
                    self.signals_out.notify("down".to_string());
                },
                piston_window::Key::Up => {
                    self.signals_out.notify("up".to_string());
                }
                _ => {},
            }
        }
        
    }
}