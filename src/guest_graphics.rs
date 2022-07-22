use std::{
    fmt,
    sync::{Arc, Mutex},
};

use crate::host_graphics;

pub fn display_loop(display_threaded: ThreadedDisplay)
{
    loop
    {
        let mut display = display_threaded.lock().unwrap();
        if display.buffer_tainted
        {
            host_graphics::Terminal::clear_terminal();
            display.debuff();
            print!("{}\r\n", display);
        }
    }
}
pub fn get_fonts() -> [Sprite; 16]
{
    let font: [Sprite; 16] = [
        Sprite {
            sprite_data: [
                0xF0, 0x90, 0x90, 0x90, 0xF0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
            height: 5,
        },
        Sprite {
            sprite_data: [
                0x20, 0x60, 0x20, 0x20, 0x70, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
            height: 5,
        },
        Sprite {
            sprite_data: [
                0xF0, 0x10, 0xF0, 0x80, 0xF0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
            height: 5,
        },
        Sprite {
            sprite_data: [
                0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
            height: 5,
        },
        Sprite {
            sprite_data: [
                0x90, 0x90, 0xF0, 0x10, 0x10, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
            height: 5,
        },
        Sprite {
            sprite_data: [
                0xF0, 0x80, 0xf0, 0x10, 0xf0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
            height: 5,
        },
        Sprite {
            sprite_data: [
                0xF0, 0x80, 0xF0, 0x90, 0xF0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
            height: 5,
        },
        Sprite {
            sprite_data: [
                0xF0, 0x10, 0x20, 0x40, 0x40, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
            height: 5,
        },
        Sprite {
            sprite_data: [
                0xF0, 0x90, 0xF0, 0x90, 0xF0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
            height: 5,
        },
        Sprite {
            sprite_data: [
                0xF0, 0x90, 0xF0, 0x10, 0xF0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
            height: 5,
        },
        Sprite {
            sprite_data: [
                0xF0, 0x90, 0xF0, 0x90, 0x90, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
            height: 5,
        },
        Sprite {
            sprite_data: [
                0xE0, 0x90, 0xE0, 0x90, 0xE0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
            height: 5,
        },
        Sprite {
            sprite_data: [
                0xF0, 0x80, 0x80, 0x80, 0xF0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
            height: 5,
        },
        Sprite {
            sprite_data: [
                0xE0, 0x90, 0x90, 0x90, 0xE0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
            height: 5,
        },
        Sprite {
            sprite_data: [
                0xF0, 0x80, 0xF0, 0x80, 0xF0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
            height: 5,
        },
        Sprite {
            sprite_data: [
                0xF0, 0x80, 0xF0, 0x80, 0x80, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
            height: 5,
        },
    ];
    font
}
pub type ThreadedDisplay = Arc<Mutex<ChipDisplay>>;

pub struct ChipDisplay
{
    pub data: [u8; 64 * 32],
    pub buffer: [u8; 64 * 32],
    pub buffer_tainted: bool,
}

impl ChipDisplay
{
    pub fn clear(&mut self)
    {
        self.buffer = [0; 64 * 32];
        self.buffer_tainted = true;
    }
    pub fn debuff(&mut self)
    {
        self.data = self.buffer;
        self.buffer_tainted = false;
    }

    pub fn get_pixel(&self, x: u8, y: u8) -> Option<u8>
    {
        self.data.get(ChipDisplay::get_buffer_position_from_x_and_y(x, y)).copied()
    }
    pub fn new() -> Self
    {
        Self {
            data: [0u8; 64 * 32],
            buffer: [0u8; 64 * 32],
            buffer_tainted: true,
        }
    }
    pub fn get_buffer_position_from_x_and_y(x: u8, y: u8) -> usize
    {
        (32 * x as usize) + y as usize
    }
    pub fn set_pixel(&mut self, x: u8, y: u8, state: bool)
    {
        self.buffer[ChipDisplay::get_buffer_position_from_x_and_y(x, y)] =
            if state { 1 } else { 0 };
        self.buffer_tainted = true;
    }
    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite: Sprite) -> bool
    {
        let mut xor_cleared_data_marker = false;
        let x = x % 64;
        let y = y % 32;
        for (sprite_y, sprite_byte) in sprite.sprite_data.iter().enumerate()
        {
            if sprite_y >= sprite.height as usize
            {
                break;
            }
            for sprite_x in 0..8
            {
                let sprite_bit = sprite_byte & (128u8 >> sprite_x) != 0;
                if sprite_bit
                {
                    let display_bit = self
                        .get_pixel(x + sprite_x as u8, y + sprite_y as u8)
                        .unwrap()
                        != 0;
                    if display_bit
                    {
                        xor_cleared_data_marker = true;
                        self.set_pixel(x + sprite_x as u8, y + sprite_y as u8, false);
                    }
                    else
                    {
                        self.set_pixel(x + sprite_x as u8, y + sprite_y as u8, true);
                    }
                }
            }
        }
        xor_cleared_data_marker
    }
}

impl Default for ChipDisplay
{
    fn default() -> Self
    {
        Self::new()
    }
}

impl fmt::Display for ChipDisplay
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        for y in 0..32
        {
            for x in 0..64
            {
                // f.write_str(self.data.get(x * y));
                write!(
                    f,
                    "{} ",
                    if self
                        .get_pixel(x, y)
                        .expect("Tried to display value outside of bounds of graphics data")
                        == 1
                    {
                        // 1
                        "⬛"
                    }
                    else
                    {
                        // 0
                        "⬜"
                    }
                )
                .unwrap();
            }
            f.write_str("\n\r").unwrap();
        }
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub struct Sprite
{
    pub sprite_data: [u8; 15],
    pub height: u8,
}

impl Sprite
{
    pub fn new(sprite_data: [u8; 15], height: u8) -> Self
    {
        Self {
            sprite_data,
            height,
        }
    }
    pub fn new_from_bytes(bytes: &[u8]) -> Sprite
    {
        let height = bytes.len();
        let mut bytes = bytes.to_vec();
        bytes.resize(15, 0x0);

        let sprite_data: [u8; 15] = bytes.try_into().unwrap();
        Self {
            sprite_data,
            height: height.try_into().unwrap(),
        }
    }
}
impl fmt::Display for Sprite
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        writeln!(f, "Debug drawing for sprite: ").unwrap();
        for (i, val) in self.sprite_data.iter().enumerate()
        {
            if i >= self.height as usize
            {
                break;
            }
            for j in 0..8
            {
                let cur_bit = val & (128 >> j) != 0;
                write!(f, "{}", if cur_bit { 1 } else { 0 }).unwrap();
            }
            writeln!(f).unwrap();
        }
        Ok(())
    }
}
