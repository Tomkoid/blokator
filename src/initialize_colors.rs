use crate::Colors;
use crate::check_no_color_env;

pub fn initialize_colors() -> Colors {
    let mut colors = Colors::new_without_colors();

    // If user runs blokator with NO_COLOR flag
    #[cfg(target_family = "unix")]
    if !check_no_color_env() {
        colors = Colors::new();
    }

    colors
}
