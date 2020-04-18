extern crate loop_utility;

use loop_utility::Config;
use loop_utility::Timing;

use std::time;

use loop_utility::executor;

fn main() {
    let config = Config {
        timing: Timing::Delay(time::Duration::from_millis(1000)),
        command: String::from("ls"),
    };

    match config.timing {
        Timing::Delay(tm) => executor::run_with_delay(tm),
        Timing::Every(tm) => executor::run_every(tm),
    }
}
