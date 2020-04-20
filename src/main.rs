extern crate loop_utility;

use loop_utility::config::Config;
use loop_utility::config::Timing;

use std::env;

use loop_utility::executor;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap();

    match config.timing {
        Timing::Delay(tm) => executor::run_with_delay(tm, config.command),
        Timing::Every(tm) => executor::run_every(tm),
    }
}
