use std::time;

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
        println!("{:?}", args.0);

        Ok(Config {
            timing: Timing::Delay(time::Duration::from_millis(1000)),
            command: args.1,
        })
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
