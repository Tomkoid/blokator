use std::process::exit;
use crate::Args;

pub fn check_allowed_function(args: &Args) {
    if args.sync {
        exit(1)
    }
}
