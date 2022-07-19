#[cfg(test)]
use std::collections::HashMap;

use crate::{get_fonts, host_graphics::Terminal, ChipDisplay};

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
    let font = get_fonts();
    println!("{}", font[0]);
}
#[test]
fn key_update_single_test()
{
    let mut term = Terminal {
        key_pressed: [false; 16],
    };
    term.key_update_loop();
    Terminal::clear_terminal();
}
