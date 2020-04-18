use std::time;

pub enum Timing {
    Every(time::Duration),
    Delay(time::Duration),
}

pub struct Config {
    pub timing: Timing,
    pub command: String,
}

pub mod executor;
