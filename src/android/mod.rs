pub mod apply;
pub mod checks;
pub mod list;
pub mod restore;

pub fn clear_line() {
    print!("\x1b[2K\r");
}
