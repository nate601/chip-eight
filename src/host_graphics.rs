use std::{io::{stdin, stdout, Write}, sync::mpsc};

use termion::{input::TermRead, raw::IntoRawMode};

pub struct Terminal
{
    pub key_pressed: [bool; 16],
}

impl Terminal
{
    pub fn new() -> Self
    {
        Self {
            key_pressed: [false; 16],
        }
    }

    pub fn clear_terminal()
    {
        // Clear screen
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
    fn get_output_into_raw() {}
    pub fn key_update_loop(&mut self, tx: mpsc::Sender<char>)
    {
        let key_bindings: [char; 16] = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ];
        let mut stdout = stdout().into_raw_mode().unwrap();
        let stdin = stdin();
        for c in stdin.keys()
        {
            if let termion::event::Key::Char(c) = c.unwrap()
            {
                if (c == 'q')
                {
                    return;
                }
                let pos = key_bindings.iter().position(|x| c == *x);
                if pos != None
                {
                    let pos = pos.unwrap();
                    self.key_pressed[pos] = true;
                    tx.send(key_bindings[pos]).unwrap();
                }
            }
        }
    }
}
