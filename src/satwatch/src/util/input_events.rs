use sdl2::event::{Event as SdlEvent, WindowEvent};
use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::MouseState;

#[derive(Copy, Clone)]
pub enum Event {
    None,
    HardStop,
    MoveLeft(bool),
    MoveRight(bool),
    MoveUp(bool),
    MoveDown(bool),
    MoveForwards(bool),
    MoveBackwards(bool),
    Rotate(f32, f32),
}

fn key_to_ours(keyopt: Option<Keycode>, keymod: Mod, pressed: bool) -> Event {
    if keymod.contains(Mod::LSHIFTMOD) {
        if let Some(key) = keyopt {
            match key {
                Keycode::W => Event::MoveUp(pressed),
                Keycode::S => Event::MoveDown(pressed),
                Keycode::A => Event::MoveLeft(pressed),
                Keycode::D => Event::MoveRight(pressed),
                _ => Event::None,
            }
        } else {
            Event::None
        }
    } else {
        if let Some(key) = keyopt {
            match key {
                Keycode::W => Event::MoveForwards(pressed),
                Keycode::S => Event::MoveBackwards(pressed),
                Keycode::A => Event::MoveLeft(pressed),
                Keycode::D => Event::MoveRight(pressed),
                _ => Event::None,
            }
        } else {
            Event::None
        }
    }
}

fn mouse_to_ours(mousestate: MouseState, xrel: i32, yrel: i32) -> Event {
    if mousestate.right() {
        Event::Rotate(xrel as f32 / 360.0, yrel as f32 / 360.0)
    } else {
        Event::None
    }
}

pub fn sdl_to_our_event(event_in: SdlEvent) -> Event {
    match event_in {
        SdlEvent::MouseMotion {
            mousestate,
            xrel,
            yrel,
            ..
        } => mouse_to_ours(mousestate, xrel, yrel),
        SdlEvent::KeyDown {
            keycode, keymod, ..
        } => key_to_ours(keycode, keymod, true),
        SdlEvent::KeyUp {
            keycode, keymod, ..
        } => key_to_ours(keycode, keymod, false),
        SdlEvent::Window { win_event, .. } => match win_event {
            WindowEvent::FocusLost => Event::HardStop,
            _ => Event::None,
        },
        _ => Event::None,
    }
}
