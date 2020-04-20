use std::process::Command;
use std::thread;
use std::time;

pub fn run_with_delay(delay: time::Duration, command: Vec<String>) {
    let mut cmd = Command::new(&command[0]);
    loop {
        let res: Vec<u8> = cmd.output().expect("Error while executing command").stdout;
        print!("{}", std::str::from_utf8(&res).unwrap());
        thread::sleep(delay);
    }
}

pub fn run_every(period: time::Duration) {
    println!("Running every {:?}", period)
    // let now = time::Instant::now();
}
