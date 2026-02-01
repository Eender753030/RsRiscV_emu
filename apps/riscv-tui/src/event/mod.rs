pub mod key;

use std::thread;
use std::time::Duration;
use std::sync::mpsc::Sender;

use anyhow::Result;

use crossterm::event::{self, Event, KeyEvent};

use key::{KeyControl, poll_key_event};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmuEvent {
    Key(KeyControl),
    Resize(u16, u16),
    Tick,
}

pub fn spawn_event_thread(tx: Sender<EmuEvent>) {
    thread::spawn(move || -> Result<()> {
        loop {
            let timeout = Duration::from_millis(100);

            if event::poll(timeout)? {
                match event::read()? {
                    Event::Key(KeyEvent{code, ..}) => {
                        if let Some(key) = poll_key_event(code) {
                            tx.send(EmuEvent::Key(key))?;
                        }  
                    }
                    Event::Resize(x, y) => tx.send(EmuEvent::Resize(x, y))?,
                    _ => {},
                }
            } else {
                tx.send(EmuEvent::Tick)?;
            }
        }
    });
}
