use std::{
    fmt,
    fs::File,
    io::Read,
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
    let display: ChipDisplay = ChipDisplay::new();
    let display_threaded: ThreadedDisplay = Arc::new(Mutex::new(display));
    let display_threaded_loop_clone: ThreadedDisplay = display_threaded;
    let _font = guest_graphics::get_fonts();
    let mut terminal = Terminal::new();

    let (tx, rx) = mpsc::channel();

    let _key_read_handle = thread::spawn(move || {
        terminal.key_update_loop(tx);
    });
    let _rendering_handle = thread::spawn(move || {
        guest_graphics::display_loop(display_threaded_loop_clone);
    });

    let mut ram: ChipRam = [0; 4096];
    let mut registers: ChipRegisters = ChipRegisters::new();

    let rom_name = "a.rom";
    load_rom_into_ram(rom_name, &mut ram);
    registers.pc = 200;

    loop
    {
        let next_instruction = Instruction::get_next_instruction(ram, &mut registers);
    }
}

struct Instruction
{
    pub data: [u8; 2],
}

impl Instruction
{
    pub fn get_next_instruction(ram: [u8; 4096], registers: &mut ChipRegisters) -> Instruction
    {
        let next_instruction = [ram[registers.pc as usize], ram[registers.pc as usize + 1]];
        registers.pc += 2;
        Instruction::new(next_instruction)
    }

    pub fn get_nnn(&self) -> u16
    {
        let mut k = 0u16;
        k |= self.data[1] as u16;
        let mut j = (self.data[0] & 0b1111) as u16;
        j <<= 8;
        k |= j;
        k
    }
    pub fn get_n(&self) -> u8
    {
        self.data[1] & 0b1111
    }
    pub fn get_x(&self) -> u8
    {
        self.data[0] & 0b1111
    }
    pub fn get_y(&self) -> u8
    {
        self.data[1] >> 4
    }
    pub fn get_kk(&self) -> u8
    {
        self.data[1]
    }

    fn new(next_instruction: [u8; 2]) -> Instruction
    {
        Self {
            data: next_instruction,
        }
    }
}

fn load_rom_into_ram(rom_name: &str, ram: &mut [u8; 4096])
{
    let mut file_handle = File::open(rom_name).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    file_handle.read_to_end(&mut buf).unwrap();
    let base_ram_position = 0x200usize;
    for (i, val) in buf.iter().enumerate()
    {
        ram[i + base_ram_position] = *val;
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
impl ChipRegisters
{
    fn new() -> ChipRegisters
    {
        Self {
            v: [0u8; 16],
            i: 0u16,
            delay: 0u8,
            sound: 0u8,
            pc: 0u16,
            sp: 0u8,
            stack: [0u16; 16],
        }
    }
}

impl Default for ChipRegisters
{
    fn default() -> Self
    {
        Self::new()
    }
}
