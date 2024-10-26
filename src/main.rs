extern crate sizr;

use sizr::Config;
use sizr::XResult::XErr;
use sizr::XResult::XExit;
use sizr::XResult::XOk;
use std::process;

fn main() {
    // handle SIGPIPE
    let _signal = unsafe { signal_hook::register(signal_hook::SIGPIPE, || process::exit(0)) };

    // Parse arguments
    let cfg = match Config::new() {
        XOk(cfg) => cfg,
        XExit => process::exit(0),
        XErr(err) => {
            eprintln!("{}", err);
            process::exit(1)
        }
    };

    // Execution
    sizr::run(&cfg);
}
