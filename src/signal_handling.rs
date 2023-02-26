#[cfg(target_family = "unix")]
use signal_hook::{consts::SIGINT, consts::SIGTERM, iterator::Signals};
use std::process::exit;
use std::{sync::Arc, sync::Mutex, thread};

use crate::initialize_colors::initialize_colors;

// Signal handling (ex: CTRL + c)
#[cfg(target_family = "unix")]
pub fn handle_signals(thread_state: Arc<Mutex<bool>>) {
    let mut signals = Signals::new([SIGTERM, SIGINT]).unwrap();

    thread::spawn(move || {
        let mut already_pressed = false;
        for _ in signals.forever() {
            if *thread_state.lock().unwrap() {
                if !already_pressed {
                    println!(
                        " {}Force kill with CTRL + C{}",
                        initialize_colors().bold_red,
                        initialize_colors().reset
                    );
                    already_pressed = true;
                    continue;
                } else {
                    exit(2);
                }
            }
            println!(
                " {}Exiting..{}",
                initialize_colors().bold_red,
                initialize_colors().reset
            );
            exit(1);
        }
    });
}
