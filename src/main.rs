use std::{
    fmt,
    ops::{BitAnd, BitXor},
};

fn main()
{
    let mut display: ChipDisplay = ChipDisplay::new();
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

    print!("{}", display);
    // display.set_pixel(5, 0, true);
    // display.set_pixel(6, 0, true);
    // display.set_pixel(7, 0, true);
    // display.set_pixel(8, 0, true);
    // display.set_pixel(9, 0, true);
    let mut font_val = 0usize;
    for i in 0..64
    {
        if i % 5 != 0
        {
            continue;
        }
        display.draw_sprite(i, 0, font[font_val]);
        font_val += 1;
    }
    loop
    {
        if display.buffer_tainted
        {
            // print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear screen
            println!();
            display.debuff();
            print!("{}", display);
            // print!("{:?}", display.data)
        }
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

struct ChipDisplay
{
    data: [u8; 64 * 32],
    buffer: [u8; 64 * 32],
    buffer_tainted: bool,
}

impl ChipDisplay
{
    pub fn debuff(&mut self)
    {
        self.data = self.buffer;
        self.buffer_tainted = false;
    }

    pub fn get_pixel(&self, x: u8, y: u8) -> Option<u8>
    {
        // println!("{}, {}", x, y);
        Some(
            ({
                let this = self
                    .data
                    .get(ChipDisplay::get_buffer_position_from_x_and_y(x, y));
                match this
                {
                    Some(val) => val,
                    None => panic!(
                        "called `Option::unwrap()` on a `None` value {}, {} = {}",
                        x,
                        y,
                        (32 * y as i32) + x as i32
                    ),
                }
            }) | {
                let this = self
                    .buffer
                    .get(ChipDisplay::get_buffer_position_from_x_and_y(x, y));
                match this
                {
                    Some(val) => val,
                    None => panic!(
                        "called `Option::unwrap()` on a `None` value {}, {} = {} ",
                        x,
                        y,
                        (32 * y as i32) + x as i32
                    ),
                }
            },
        )
    }
    fn new() -> Self
    {
        Self {
            data: [0u8; 64 * 32],
            buffer: [0u8; 64 * 32],
            buffer_tainted: false,
        }
    }
    fn get_buffer_position_from_x_and_y(x: u8, y: u8) -> usize
    {
        (32 * x as usize) + y as usize
    }
    fn set_pixel(&mut self, x: u8, y: u8, state: bool)
    {
        let cur_val = self.get_pixel(x, y).unwrap();
        self.buffer[ChipDisplay::get_buffer_position_from_x_and_y(x, y)] =
            if state { 1 } else { 0 };
        self.buffer_tainted = true;
    }
    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite: Sprite) -> bool
    {
        let mut ret_val = false;
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
                    if sprite_bit == display_bit
                    {
                        ret_val = true;
                        self.set_pixel(x + sprite_x as u8, y + sprite_y as u8, false);
                    }
                    else
                    {
                        self.set_pixel(x + sprite_x as u8, y + sprite_y as u8, true);
                    }
                }
            }
        }
        ret_val
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
                        "▓"
                    }
                    else
                    {
                        // 0
                        "▁"
                    }
                )
                .unwrap();
            }
            f.write_str("\n").unwrap();
        }
        Ok(())
    }
}

#[derive(Copy, Clone)]
struct Sprite
{
    sprite_data: [u8; 15],
    height: u8,
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
#[cfg(test)]
mod tests
{
    use std::collections::{HashMap};

    use crate::{ChipDisplay, Sprite};

    #[test]
    fn check_if_collision_in_buffer_and_x_y_test()
    {
        let mut hs: HashMap<usize, (u8, u8)> = HashMap::new();
        for x in 0..64
        {
            for y in 0..32
            {
                let buf_pos = ChipDisplay::get_buffer_position_from_x_and_y(x, y);
                // println!("{}, {} = {}", x, y, buf_pos);
                if let std::collections::hash_map::Entry::Vacant(e) = hs.entry(buf_pos)
                {
                    e.insert((x, y));
                }
                else
                {
                    let (hs_x, hs_y) = *hs.get(&buf_pos).unwrap();
                    panic!(
                        "{}, {} = {} collides with {},{} ={}",
                        x, y, buf_pos, hs_x, hs_y, buf_pos
                    );
                }
            }
        }
    }

    #[test]
    fn sprite_print_test()
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
                    0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
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
        println!("{}", font[0]);
    }
}
