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
                OpCode::LdVxBy =>
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
                    let sprite_data = &ram[registers.i as usize
                        ..next_instruction.get_n() as usize + registers.i as usize];
                    let xpos = registers.v[next_instruction.get_x() as usize];
                    let ypos = registers.v[next_instruction.get_y() as usize];
                    let sprite = Sprite::new_from_bytes(sprite_data);
                    let xor = dis.draw_sprite(xpos, ypos, sprite);
                    registers.v[0xFusize] = if xor { 1 } else { 0 };
                }
                OpCode::Ret => todo!(),
                OpCode::Call => todo!(),
                OpCode::SeVxBy => todo!(),
                OpCode::SneVxBy => todo!(),
                OpCode::SeVxVy => todo!(),
                OpCode::AddVxBy => todo!(),
                OpCode::LdVxVy => todo!(),
                OpCode::OrVxVy => todo!(),
                OpCode::AndVxVy => todo!(),
                OpCode::XorVxVy => todo!(),
                OpCode::AddVxVy => todo!(),
                OpCode::SubVxVy => todo!(),
                OpCode::ShrVxVy => todo!(),
                OpCode::SubnVxVy => todo!(),
                OpCode::ShlVxVy => todo!(),
                OpCode::SneVxVy => todo!(),
                OpCode::JpV0Addr => todo!(),
                OpCode::RndVxBy => todo!(),
                OpCode::SkpVx => todo!(),
                OpCode::SknpVx => todo!(),
                OpCode::LdVxDt => todo!(),
                OpCode::LdVxK => todo!(),
                OpCode::LdDtVx => todo!(),
                OpCode::LdStVx => todo!(),
                OpCode::AddIVx => todo!(),
                OpCode::LdFVx => todo!(),
                OpCode::LdBVx => todo!(),
                OpCode::LdIVx => todo!(),
                OpCode::LdVxI => todo!(),
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
    definition: Vec<OperationComponent>,
    op_code: OpCode,
}

impl Operation
{
    fn new(op_code: OpCode, definition: Vec<OperationComponent>) -> Self
    {
        Self {
            definition,
            op_code,
        }
    }

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
                definition: vec![
                    OperationComponent::Literal(0x0),
                    OperationComponent::Literal(0x0),
                    OperationComponent::Literal(0xE),
                    OperationComponent::Literal(0x0),
                ],
                op_code: OpCode::Cls,
            },
            Self {
                definition: vec![OperationComponent::Literal(0x1), OperationComponent::Nnn],
                op_code: OpCode::Jmp,
            },
            Self {
                definition: vec![
                    OperationComponent::Literal(0x6),
                    OperationComponent::X,
                    OperationComponent::Kk,
                ],
                op_code: OpCode::LdVxBy,
            },
            Self {
                definition: vec![
                    OperationComponent::Literal(0x7),
                    OperationComponent::X,
                    OperationComponent::Kk,
                ],
                op_code: OpCode::Add,
            },
            Self {
                definition: vec![OperationComponent::Literal(0xA), OperationComponent::Nnn],
                op_code: OpCode::LdI,
            },
            Self {
                definition: vec![
                    OperationComponent::Literal(0xD),
                    OperationComponent::X,
                    OperationComponent::Y,
                    OperationComponent::N,
                ],
                op_code: OpCode::Display,
            },
            Self::new(
                OpCode::Ret,
                vec![
                    OperationComponent::Literal(0x0),
                    OperationComponent::Literal(0x0),
                    OperationComponent::Literal(0xE),
                    OperationComponent::Literal(0xE),
                ],
            ),
            Self::new(
                OpCode::Call,
                vec![OperationComponent::Literal(0x2), OperationComponent::Nnn],
            ),
            Self::new(
                OpCode::SeVxBy,
                vec![
                    OperationComponent::Literal(0x3),
                    OperationComponent::X,
                    OperationComponent::Kk,
                ],
            ),
            Self::new(
                OpCode::SneVxBy,
                vec![
                    OperationComponent::Literal(0x4),
                    OperationComponent::X,
                    OperationComponent::Kk,
                ],
            ),
            Self::new(
                OpCode::SeVxVy,
                vec![
                    OperationComponent::Literal(0x5),
                    OperationComponent::X,
                    OperationComponent::Y,
                    OperationComponent::Literal(0x0),
                ],
            ),
            Self::new(
                OpCode::AddVxBy,
                vec![
                    OperationComponent::Literal(0x7),
                    OperationComponent::X,
                    OperationComponent::Kk,
                ],
            ),
            Self::new(
                OpCode::LdVxVy,
                vec![
                    OperationComponent::Literal(0x8),
                    OperationComponent::X,
                    OperationComponent::Y,
                    OperationComponent::Literal(0x0),
                ],
            ),
            Self::new(
                OpCode::OrVxVy,
                vec![
                    OperationComponent::Literal(0x8),
                    OperationComponent::X,
                    OperationComponent::Y,
                    OperationComponent::Literal(0x1),
                ],
            ),
            Self::new(
                OpCode::AddVxVy,
                vec![
                    OperationComponent::Literal(0x8),
                    OperationComponent::X,
                    OperationComponent::Y,
                    OperationComponent::Literal(0x2),
                ],
            ),
            Self::new(
                OpCode::XorVxVy,
                vec![
                    OperationComponent::Literal(0x8),
                    OperationComponent::X,
                    OperationComponent::Y,
                    OperationComponent::Literal(0x3),
                ],
            ),
            Self::new(
                OpCode::AndVxVy,
                vec![
                    OperationComponent::Literal(0x8),
                    OperationComponent::X,
                    OperationComponent::Y,
                    OperationComponent::Literal(0x4),
                ],
            ),
            Self::new(
                OpCode::SubVxVy,
                vec![
                    OperationComponent::Literal(0x8),
                    OperationComponent::X,
                    OperationComponent::Y,
                    OperationComponent::Literal(0x5),
                ],
            ),
            Self::new(
                OpCode::ShrVxVy,
                vec![
                    OperationComponent::Literal(0x8),
                    OperationComponent::X,
                    OperationComponent::Y,
                    OperationComponent::Literal(0x6),
                ],
            ),
            Self::new(
                OpCode::SubnVxVy,
                vec![
                    OperationComponent::Literal(0x8),
                    OperationComponent::X,
                    OperationComponent::Y,
                    OperationComponent::Literal(0x7),
                ],
            ),
            Self::new(
                OpCode::ShlVxVy,
                vec![
                    OperationComponent::Literal(0x8),
                    OperationComponent::X,
                    OperationComponent::Y,
                    OperationComponent::Literal(0xE),
                ],
            ),
            Self::new(
                OpCode::SneVxVy,
                vec![
                    OperationComponent::Literal(0x9),
                    OperationComponent::X,
                    OperationComponent::Y,
                    OperationComponent::Literal(0x0),
                ],
            ),
            Self::new(
                OpCode::JpV0Addr,
                vec![OperationComponent::Literal(0xB), OperationComponent::Nnn],
            ),
            Self::new(
                OpCode::RndVxBy,
                vec![
                    OperationComponent::Literal(0xC),
                    OperationComponent::X,
                    OperationComponent::Kk,
                ],
            ),
            Self::new(
                OpCode::SkpVx,
                vec![
                    OperationComponent::Literal(0xE),
                    OperationComponent::X,
                    OperationComponent::Literal(0x9),
                    OperationComponent::Literal(0xE),
                ],
            ),
            Self::new(
                OpCode::SknpVx,
                vec![
                    OperationComponent::Literal(0xE),
                    OperationComponent::X,
                    OperationComponent::Literal(0xA),
                    OperationComponent::Literal(0x1),
                ],
            ),
            Self::new(
                OpCode::LdVxDt,
                vec![
                    OperationComponent::Literal(0xF),
                    OperationComponent::X,
                    OperationComponent::Literal(0x0),
                    OperationComponent::Literal(0x7),
                ],
            ),
            Self::new(
                OpCode::LdVxK,
                vec![
                    OperationComponent::Literal(0xF),
                    OperationComponent::X,
                    OperationComponent::Literal(0x0),
                    OperationComponent::Literal(0xA),
                ],
            ),
            Self::new(
                OpCode::LdDtVx,
                vec![
                    OperationComponent::Literal(0xF),
                    OperationComponent::X,
                    OperationComponent::Literal(0x1),
                    OperationComponent::Literal(0x5),
                ],
            ),
            Self::new(
                OpCode::LdStVx,
                vec![
                    OperationComponent::Literal(0xF),
                    OperationComponent::X,
                    OperationComponent::Literal(0x1),
                    OperationComponent::Literal(0x8),
                ],
            ),
            Self::new(
                OpCode::AddIVx,
                vec![
                    OperationComponent::Literal(0xF),
                    OperationComponent::X,
                    OperationComponent::Literal(0x1),
                    OperationComponent::Literal(0xE),
                ],
            ),
            Self::new(
                OpCode::LdFVx,
                vec![
                    OperationComponent::Literal(0xF),
                    OperationComponent::X,
                    OperationComponent::Literal(0x2),
                    OperationComponent::Literal(0x9),
                ],
            ),
            Self::new(
                OpCode::LdBVx,
                vec![
                    OperationComponent::Literal(0xF),
                    OperationComponent::X,
                    OperationComponent::Literal(0x3),
                    OperationComponent::Literal(0x3),
                ],
            ),
            Self::new(
                OpCode::LdIVx,
                vec![
                    OperationComponent::Literal(0xF),
                    OperationComponent::X,
                    OperationComponent::Literal(0x5),
                    OperationComponent::Literal(0x5),
                ],
            ),
            Self::new(
                OpCode::LdVxI,
                vec![
                    OperationComponent::Literal(0xF),
                    OperationComponent::X,
                    OperationComponent::Literal(0x6),
                    OperationComponent::Literal(0x5),
                ],
            ),
        ]
    }
}
#[derive(Copy, Clone)]
enum OpCode
{
    Cls,
    Jmp,
    LdVxBy,
    Add,
    LdI,
    Display,
    Ret,
    Call,
    SeVxBy,
    SneVxBy,
    SeVxVy,
    AddVxBy,
    LdVxVy,
    OrVxVy,
    AndVxVy,
    XorVxVy,
    AddVxVy,
    SubVxVy,
    ShrVxVy,
    SubnVxVy,
    ShlVxVy,
    SneVxVy,
    JpV0Addr,
    RndVxBy,
    SkpVx,
    SknpVx,
    LdVxDt,
    LdVxK,
    LdDtVx,
    LdStVx,
    AddIVx,
    LdFVx,
    LdBVx,
    LdIVx,
    LdVxI,
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
