use std::process::Command;
use std::thread;
use std::time;

pub fn run_with_delay(delay: time::Duration, command: Vec<String>) {
    loop {
        run_command(&command);
        thread::sleep(delay);
    }
}

pub fn run_every(period: time::Duration, command: Vec<String>) {
    loop {
        let next_iteration = get_next_time_instant(period);
        sleep_until(next_iteration);
        run_command(&command);
    }
}

fn run_command(command: &Vec<String>) {
    let program = &command[0];
    let args = &command[1..];
    println!("{:?}, {:?}", program, args);
    let mut cmd = Command::new(program);
    if args.len() > 0 {
        cmd.args(args);
    }
    let res: Vec<u8> = cmd.output().expect("Error while executing command").stdout;
    print!("{}", std::str::from_utf8(&res).unwrap());
}

fn get_next_time_instant(duration: time::Duration) -> time::SystemTime {
    let now = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .expect("Error while reading current time")
        .as_millis();
    let next = now - (now % duration.as_millis()) + duration.as_millis();
    assert!(next > now);
    println!("{:?} {:?}", now, next);
    time::UNIX_EPOCH + time::Duration::from_millis(next as u64)
}

fn sleep_until(t: time::SystemTime) {
    let now = time::SystemTime::now();
    let diff = t.duration_since(now);
    match diff {
        Ok(duration) => thread::sleep(duration),
        Err(_) => {}
    }
}
