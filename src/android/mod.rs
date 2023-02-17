pub mod apply;
pub mod restore;
pub mod checks;
pub mod list;

pub fn clear_line() {
    print!("\x1b[2K\r");
}
