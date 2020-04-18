use std::process::Command;
use std::thread;
use std::time;

pub fn run_with_delay(delay: time::Duration) {
    let mut cmd = Command::new("date");
    loop {
        let res: Vec<u8> = cmd.output().expect("Error while executing command").stdout;
        print!("{}", std::str::from_utf8(&res).unwrap());
        thread::sleep(delay);
    }
}

pub fn run_every(period: time::Duration) {
    println!("Running every")
    // let now = time::Instant::now();
}
