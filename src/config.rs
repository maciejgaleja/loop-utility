use std::time;

use structopt::StructOpt;

pub enum Timing {
    Every(time::Duration),
    Delay(time::Duration),
}

pub struct Config {
    pub timing: Timing,
    pub command: Vec<String>,
}

#[derive(StructOpt)]
struct Cli {
    loop_type: String,
    time_spec: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Timing, &'static str> {
        let parsed_args = Cli::from_iter(args);

        let millis = (parsed_args.time_spec.parse::<f32>().unwrap() * 1000.0) as u64;

        match &parsed_args.loop_type[..] {
            "every" => Ok(Timing::Every(time::Duration::from_millis(millis))),
            "delay" => Ok(Timing::Delay(time::Duration::from_millis(millis))),
            _ => Err("invalid loop type"),
        }
    }

    pub fn print_help() {
        Cli::clap().print_help();
    }
}
