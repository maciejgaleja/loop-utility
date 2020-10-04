extern crate loop_utility;

use loop_utility::arg_separator;
use loop_utility::config::Config;
use loop_utility::config::Timing;
use loop_utility::executor;

use std::env;

fn main() {
    let cli_args: Vec<String> = env::args().collect();

    let (args, cmd) = arg_separator::split_args(&cli_args);

    println!("{:?}", args);

    let timing = Config::new(&args);
    match timing {
        Ok(timing) => match timing {
            Timing::Delay(tm) => executor::run_with_delay(tm, cmd),
            Timing::Every(tm) => executor::run_every(tm),
        },
        Err(msg) => {
            println!("{}", msg);
            Config::print_help()
        }
    };
}
