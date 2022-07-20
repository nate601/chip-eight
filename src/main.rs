use std::{
    fmt,
    fs::File,
    io::Read,
    ops::Deref,
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
    let display_threaded_loop_clone: ThreadedDisplay = display_threaded.clone();
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
    registers.pc = 0x200;

    loop
    {
        thread::sleep(Duration::from_millis(1000));
        let next_instruction = Instruction::get_next_instruction(ram, &mut registers);
        let next_op = Operation::get_op_code(&next_instruction);
        if let Some(x) = next_op
        {
            match x
            {
                OpCode::Cls => display_threaded.lock().unwrap().clear(),
                OpCode::Jmp => registers.pc = next_instruction.get_nnn(),
                OpCode::Ld =>
                {
                    registers.v[next_instruction.get_x() as usize] = next_instruction.get_kk()
                }
                OpCode::Add =>
                {
                    registers.v[next_instruction.get_x() as usize] += next_instruction.get_kk()
                }
                OpCode::LdI => registers.i = next_instruction.get_nnn(),
                OpCode::Display =>
                {
                    let mut dis = display_threaded.lock().unwrap();
                    let sprite_data = &ram[registers.i as usize..next_instruction.get_n() as usize + registers.i as usize];
                    let xpos = registers.v[next_instruction.get_x() as usize];
                    let ypos = registers.v[next_instruction.get_y() as usize];
                    let sprite = Sprite::new_from_bytes(sprite_data);
                    let xor = dis.draw_sprite(xpos, ypos, sprite);
                    registers.v[0xFusize] = if xor { 1 } else { 0 };
                }
            }
        }
        else
        {
            panic!("Unknown op! {}", next_instruction);
        }
    }
}

struct Operation
{
    name: String,
    definition: Vec<OperationComponent>,
    op_code: OpCode,
}

impl Operation
{
    pub fn get_op_code(instruction: &Instruction) -> Option<OpCode>
    {
        let operations = Operation::get_operations();
        let instruction_bits = instruction.get_bits();
        let matches = operations
            .iter()
            .filter(|cur_op| {
                let mut i = 0usize;
                for comp in &cur_op.definition
                {
                    match comp
                    {
                        OperationComponent::Literal(c) =>
                        {
                            if instruction_bits[i] != *c
                            {
                                return false;
                            }
                            i += 1;
                        }
                        OperationComponent::Nnn => i += 3,
                        OperationComponent::N => i += 1,
                        OperationComponent::X => i += 1,
                        OperationComponent::Y => i += 1,
                        OperationComponent::Kk => i += 2,
                    }
                }
                true
            })
            .map(|f| f.op_code)
            .collect::<Vec<OpCode>>();
        matches.first().copied()
    }

    pub fn get_operations() -> Vec<Operation>
    {
        vec![
            Self {
                name: "CLS".to_string(),
                definition: vec![
                    OperationComponent::Literal(0x0),
                    OperationComponent::Literal(0x0),
                    OperationComponent::Literal(0xE),
                    OperationComponent::Literal(0x0),
                ],
                op_code: OpCode::Cls,
            },
            Self {
                name: "JMP".to_string(),
                definition: vec![OperationComponent::Literal(0x1), OperationComponent::Nnn],
                op_code: OpCode::Jmp,
            },
            Self {
                name: "LD Vx, byte".to_string(),
                definition: vec![
                    OperationComponent::Literal(0x6),
                    OperationComponent::X,
                    OperationComponent::Kk,
                ],
                op_code: OpCode::Ld,
            },
            Self {
                name: "ADD Vx, byte".to_string(),
                definition: vec![
                    OperationComponent::Literal(0x7),
                    OperationComponent::X,
                    OperationComponent::Kk,
                ],
                op_code: OpCode::Add,
            },
            Self {
                name: "LD I, addr".to_string(),
                definition: vec![OperationComponent::Literal(0xA), OperationComponent::Nnn],
                op_code: OpCode::LdI,
            },
            Self {
                name: "Display Sprite".to_string(),
                definition: vec![
                    OperationComponent::Literal(0xD),
                    OperationComponent::X,
                    OperationComponent::Y,
                    OperationComponent::N,
                ],
                op_code: OpCode::Display,
            },
        ]
    }
}
#[derive(Copy, Clone)]
enum OpCode
{
    Cls,
    Jmp,
    Ld,
    Add,
    LdI,
    Display,
}
#[derive(Copy, Clone)]
enum OperationComponent
{
    Literal(u8),
    Nnn,
    N,
    X,
    Y,
    Kk,
}

#[derive(Debug)]
struct Instruction
{
    pub data: [u8; 2],
}

impl fmt::Display for Instruction
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "\r\n Instruction display:").unwrap();
        for i in self.data
        {
            write!(f, "\r\n {:x?}", i).unwrap();
        }
        Ok(())
    }
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
        let mut k = self.data[1] as u16;
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
    pub fn get_bits(&self) -> [u8; 4]
    {
        [
            self.data[0] >> 4,
            self.data[0] & 0b1111,
            self.data[1] >> 4,
            self.data[1] & 0b1111,
        ]
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
