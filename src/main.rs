use std::{
    fmt,
    fs::File,
    io::Read,
    ops::Deref,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

use host_graphics::Input;

use crate::guest_graphics::*;
use crate::host_graphics::Terminal;

pub mod guest_graphics;
mod host_graphics;
pub mod tests;

const OPS_PER_SECOND: u64 = 1000;

fn main()
{
    let display: ChipDisplay = ChipDisplay::new();
    let display_threaded: ThreadedDisplay = Arc::new(Mutex::new(display));
    let display_threaded_loop_clone: ThreadedDisplay = display_threaded.clone();
    let _font = guest_graphics::get_fonts();
    let mut terminal = Terminal::new();
    let input_threaded = Input::get_threaded_input();
    let input_threaded_clone = input_threaded.clone();

    let (tx, rx) = mpsc::channel();

    let _key_read_handle = thread::spawn(move || {
        terminal.key_update_loop(tx, input_threaded_clone);
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
        thread::sleep(Duration::from_millis(1 / OPS_PER_SECOND));
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
                OpCode::Ret =>
                {
                    registers.pc = registers.stack[registers.sp as usize];
                    registers.sp -= 1;
                }
                OpCode::Call =>
                {
                    registers.sp += 1;
                    registers.stack[registers.sp as usize] = registers.pc;
                    registers.pc = next_instruction.get_nnn();
                }
                OpCode::SeVxBy =>
                {
                    if registers.v[next_instruction.get_x() as usize] == next_instruction.get_kk()
                    {
                        registers.pc += 2;
                    }
                }
                OpCode::SneVxBy =>
                {
                    if registers.v[next_instruction.get_x() as usize] != next_instruction.get_kk()
                    {
                        registers.pc += 2;
                    }
                }
                OpCode::SeVxVy =>
                {
                    if registers.v[next_instruction.get_x() as usize]
                        == registers.v[next_instruction.get_y() as usize]
                    {
                        registers.pc += 2;
                    }
                }
                OpCode::AddVxBy =>
                {
                    let k = registers.v[next_instruction.get_x() as usize];
                    registers.v[next_instruction.get_x() as usize] = next_instruction.get_kk() + k;
                }
                OpCode::LdVxVy =>
                {
                    registers.v[next_instruction.get_x() as usize] =
                        registers.v[next_instruction.get_y() as usize];
                }
                OpCode::OrVxVy =>
                {
                    registers.v[next_instruction.get_x() as usize] |=
                        registers.v[next_instruction.get_y() as usize];
                }
                OpCode::AndVxVy =>
                {
                    registers.v[next_instruction.get_x() as usize] &=
                        registers.v[next_instruction.get_y() as usize];
                }
                OpCode::XorVxVy =>
                {
                    registers.v[next_instruction.get_x() as usize] ^=
                        registers.v[next_instruction.get_y() as usize];
                }
                OpCode::AddVxVy =>
                {
                    let x = registers.v[next_instruction.get_x() as usize] as u16;
                    let y = registers.v[next_instruction.get_y() as usize] as u16;
                    let mut val = x + y;
                    let mut flag = false;
                    if val > 255
                    {
                        flag = true;
                        val &= 0b1111_1111;
                    }
                    registers.v[next_instruction.get_x() as usize] = val as u8;
                    registers.v[0xF] = if flag { 1 } else { 0 };
                }
                OpCode::SubVxVy =>
                {
                    let x = registers.v[next_instruction.get_x() as usize];
                    let y = registers.v[next_instruction.get_y() as usize];
                    let mut flag = false;
                    if x > y
                    {
                        flag = true;
                    }
                    let val = x - y;
                    registers.v[next_instruction.get_x() as usize] = val;
                    registers.v[0xF] = if flag { 1 } else { 0 };
                }
                OpCode::ShrVxVy =>
                {
                    let x = registers.v[next_instruction.get_x() as usize];
                    let mut flag = false;
                    let mut val = x;
                    if x & 0b1 == 1
                    {
                        flag = true;
                    }
                    val /= 2;
                    registers.v[next_instruction.get_x() as usize] = val;
                    registers.v[0xF] = if flag { 1 } else { 0 };
                }
                OpCode::SubnVxVy =>
                {
                    let x = registers.v[next_instruction.get_x() as usize];
                    let y = registers.v[next_instruction.get_y() as usize];
                    let mut flag = false;
                    let mut val = y;
                    if y > x
                    {
                        flag = true;
                    }
                    val -= x;
                    registers.v[next_instruction.get_x() as usize] = val;
                    registers.v[0xF] = if flag { 1 } else { 0 };
                }
                OpCode::ShlVxVy =>
                {
                    let x = registers.v[next_instruction.get_x() as usize];
                    let mut flag = false;
                    let mut val = x;
                    if x & 0b1000_0000 == 0b1000_0000
                    {
                        flag = true;
                    }
                    val *= 2;
                    registers.v[next_instruction.get_x() as usize] = val;
                    registers.v[0xF] = if flag { 1 } else { 0 };
                }
                OpCode::SneVxVy =>
                {
                    if registers.v[next_instruction.get_x() as usize]
                        != registers.v[next_instruction.get_y() as usize]
                    {
                        registers.pc += 2;
                    }
                }
                OpCode::JpV0Addr =>
                {
                    let addt = registers.v[0x0];
                    registers.pc = next_instruction.get_nnn() + addt as u16;
                }
                OpCode::RndVxBy =>
                {
                    //TODO: Implement real random number generator!
                    let random_number = 4;
                    registers.v[next_instruction.get_x() as usize] =
                        random_number & next_instruction.get_kk();
                }
                OpCode::SkpVx => {
                    let inp = input_threaded.lock().unwrap();
                    let key_index = registers.v[next_instruction.get_x() as usize] as usize;
                    if inp.key_is_down(key_index)
                    {
                        registers.pc += 2;
                    }
                },
                OpCode::SknpVx =>{
                    let inp = input_threaded.lock().unwrap();
                    let key_index = registers.v[next_instruction.get_x() as usize] as usize;
                    if !inp.key_is_down(key_index)
                    {
                        registers.pc += 2;
                    }
                },
                OpCode::LdVxDt =>
                {
                    registers.v[next_instruction.get_x() as usize] = registers.delay;
                }
                OpCode::LdVxK =>{
                    
                    let inp = input_threaded.lock().unwrap();
                    let key_index = registers.v[next_instruction.get_x() as usize] as usize;
                    if !inp.key_is_down(key_index)
                    {
                        registers.pc -= 2;
                    }
                },
                OpCode::LdDtVx => registers.delay = registers.v[next_instruction.get_x() as usize],
                OpCode::LdStVx =>
                {
                    registers.sound = registers.v[next_instruction.get_x() as usize];
                }
                OpCode::AddIVx =>
                {
                    registers.i += registers.v[next_instruction.get_x() as usize] as u16;
                }
                OpCode::LdFVx => todo!(),
                OpCode::LdBVx =>
                {
                    let x = registers.v[next_instruction.get_x() as usize];
                    let location = registers.i;
                    let hundreds = x / 100;
                    let tens = (x - (hundreds * 100)) / 10;
                    let ones = x - (hundreds * 100) - (tens * 10);

                    ram[location as usize] = hundreds;
                    ram[location as usize + 1] = tens;
                    ram[location as usize + 2] = ones;
                }
                OpCode::LdIVx =>
                {
                    let i = registers.i as usize;
                    let maxx = next_instruction.get_x()as usize;
                    ram[i..i + maxx].copy_from_slice(&registers.v[..maxx]);
                }
                OpCode::LdVxI =>
                {
                    let i = registers.i as usize;
                    let maxx = next_instruction.get_x() as usize;
                    registers.v[..maxx].copy_from_slice(&ram[i..i + maxx]);
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
    sp: i8,
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
            sp: -1i8,
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
