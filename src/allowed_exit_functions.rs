use crate::Args;
use std::process::exit;

pub fn check_allowed_function(args: &Args) {
    if args.sync {
        exit(1)
    }
}
