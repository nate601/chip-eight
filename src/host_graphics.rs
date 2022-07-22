use std::{
    io::{stdin, stdout, Write},
    sync::{mpsc, Arc, Mutex},
    time::Instant,
};

use termion::{input::TermRead, raw::IntoRawMode};

const MIN_MILLISEC_KEY_CONSIDERED_PRESSED: u128 = 100;

pub struct Terminal
{
    pub key_pressed: [bool; 16],
}

pub struct Input
{
    time_last_pressed: [Option<Instant>; 16],
}

impl Input
{
    pub fn get_threaded_input() -> ThreadedInput
    {
        let inp = Input {
            time_last_pressed: [None; 16],
        };
        Arc::new(Mutex::new(inp))
    }
    pub fn key_is_down(&self, key: usize) -> bool
    {
        if let Some(t) = self.time_last_pressed[key]
        {
            Instant::now().duration_since(t).as_millis() <= MIN_MILLISEC_KEY_CONSIDERED_PRESSED
        }
        else
        {
            false
        }
    }
}
pub type ThreadedInput = Arc<Mutex<Input>>;

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
    pub fn key_update_loop(&mut self, tx: mpsc::Sender<usize>, input: &mut ThreadedInput)
    {
        //TODO: Change these to more... ergonomic bindings
        let key_bindings: [char; 16] = [
            'x', '1', '2', '3', 'q', 'w', 'e', 'a', 's', 'd', 'z', 'c', '4', 'r', 'f', 'v',
        ];
        let mut _stdout = stdout().into_raw_mode().unwrap();
        let stdin = stdin();
        for c in stdin.keys()
        {
            if let termion::event::Key::Char(c) = c.unwrap()
            {
                if c == 'm'
                {
                    panic!("Quitting!");
                    return;
                }
                let pos = key_bindings.iter().position(|x| c == *x);
                if pos != None
                {
                    let pos = pos.unwrap();
                    self.key_pressed[pos] = true;
                    let mut inp = input.lock().unwrap();
                    inp.time_last_pressed[pos] = Some(Instant::now());

                    tx.send(pos).unwrap();
                }
            }
        }
    }
}
