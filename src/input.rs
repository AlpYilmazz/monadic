use bevy::prelude::{EventWriter, Plugin, CoreStage};
use crossterm::event::{Event, KeyCode, KeyModifiers, MouseEventKind};


const WAIT_FOR_INPUT_MS: u64 = 5;

pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

pub struct MouseEvent {
    pub kind: MouseEventKind,
    pub column: u16,
    pub row: u16,
    pub modifiers: KeyModifiers,
}

pub struct ResizeEvent { pub h: u16, pub w: u16 }

pub fn receive_terminal_inputs(
    mut key_ew: EventWriter<KeyEvent>,
    mut mouse_ew: EventWriter<MouseEvent>,
    mut resize_ew: EventWriter<ResizeEvent>,
) {
    println!("input");
    if crossterm::event::poll(std::time::Duration::from_millis(WAIT_FOR_INPUT_MS)).unwrap_or(false) {
        println!("polled");
        match crossterm::event::read() {
            Ok(event) => {
                println!("event write {:?}", event);
                match event {
                    Event::Key(k) => key_ew.send(KeyEvent {
                        code: k.code,
                        modifiers: k.modifiers
                    }),
                    Event::Mouse(m) => mouse_ew.send(MouseEvent {
                        kind: m.kind,
                        column: m.column,
                        row: m.row,
                        modifiers: m.modifiers,
                    }),
                    Event::Resize(h, w) => resize_ew.send(ResizeEvent { h, w }),
                }
            },
            Err(_) => {
                // TODO: Error handling
                println!("input read error")
            },
        }
    }
}


pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_event::<KeyEvent>()
            .add_event::<MouseEvent>()
            .add_event::<ResizeEvent>()
            .add_system_to_stage(CoreStage::PreUpdate, receive_terminal_inputs);
    }
}