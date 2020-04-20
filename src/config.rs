use std::time;

extern crate clap;
use clap::{App, Arg};

pub enum Timing {
    Every(time::Duration),
    Delay(time::Duration),
}

pub struct Config {
    pub timing: Timing,
    pub command: Vec<String>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let args = Config::get_arguments(args);

        if (args.1.len() == 0) {
            Err("No program to call")
        }
        else
        {
        let matches = App::new("loop")
            .author("Maciej G")
            .about("Does things repeatedly")
            .arg(
                Arg::with_name("every")
                    .short("e")
                    .long("every")
                    .value_name("SECONDS")
                    .help("Perform command every SECONDS seconds")
                    .takes_value(true)
                    .group("mode"),
            )
            .arg(
                Arg::with_name("delay")
                    .short("d")
                    .long("delay")
                    .value_name("SECONDS")
                    .help("Perform command with SECONDS seconds delay")
                    .takes_value(true)
                    .group("mode"),
            )
            .get_matches_from(args.0);

        let every = matches.value_of("every");
        match every {
            Some(v) => Ok(Config {
                timing: Timing::Every(time::Duration::from_millis((v.parse::<f32>().unwrap() * 1000.0) as u64)),
                command: args.1,
            }),
            None => {
                let delay = matches.value_of("delay");
                match delay {
                    Some(v) => Ok(Config {
                        timing: Timing::Delay(time::Duration::from_millis((v.parse::<f32>().unwrap() * 1000.0) as u64)),
                        command: args.1,
                    }),
                    None => Err("error"),
                }
            },
        }
    }
    }

    fn get_arguments(args: &[String]) -> (Vec<String>, Vec<String>) {
        let mut options = Vec::new();
        let mut command = Vec::new();

        let mut cmd_begin: bool = false;
        for arg in args {
            if arg == "--" {
                cmd_begin = true;
            } else {
                if cmd_begin {
                    command.push(arg.clone());
                } else {
                    options.push(arg.clone());
                }
            }
        }

        (options, command)
    }
}
