use std::{
    fmt,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

use crate::guest_graphics::*;
use crate::host_graphics::Terminal;

pub mod guest_graphics;
mod host_graphics;
pub mod tests;

fn main()
{
    let mut display: ChipDisplay = ChipDisplay::new();
    let display_threaded: ThreadedDisplay = Arc::new(Mutex::new(display));
    let display_threaded_loop_clone: ThreadedDisplay = display_threaded.clone();
    let font = guest_graphics::get_fonts();
    let mut terminal = Terminal::new();

    let (tx, rx) = mpsc::channel();

    let _key_read_handle = thread::spawn(move || {
        terminal.key_update_loop(tx);
    });
    let _rendering_handle = thread::spawn(move || {
        guest_graphics::display_loop(display_threaded_loop_clone);
    });

    loop
    {
        let rec = rx.recv().unwrap();
    }
}

type ChipRam = [u8; 4096];

struct ChipRegisters
{
    v: [u8; 16],
    i: u16,
    delay: u8,
    sound: u8,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
}
