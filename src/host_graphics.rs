pub struct Terminal {}

impl Terminal
{
    pub fn clear_terminal()
    {
        // Clear screen
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
}
