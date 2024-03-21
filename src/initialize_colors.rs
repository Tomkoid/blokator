// use crate::Colors;
//
// #[cfg(target_family = "unix")]
// use crate::colors::check_no_color_env;
//
// pub fn initialize_colors() -> Colors {
//     #[cfg(target_family = "windows")]
//     return Colors::new_without_colors();
//
//     #[cfg(target_family = "unix")]
//     {
//         let mut colors = Colors::get_colors();
//
//         // If user runs blokator with NO_COLOR flag
//         #[cfg(target_family = "unix")]
//         if !check_no_color_env() {
//             colors = Colors::new();
//         }
//
//         colors
//     }
// }
