extern crate signal_hook;

use std::process::{Command, Stdio};
use std::io;
use std::io::Write;
use std::sync::{atomic::AtomicBool, atomic::Ordering, Arc};
use std::error::Error;

fn is_special_command(buffer: &str) -> bool {
    if let Some(c) = buffer.chars().nth(0) {
        if c == ':' {
            return true
        }
    }
    false
}

fn main() -> Result<(), Box<Error>> {
    println!("Starting term-wrap");
    let should_stop = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::SIGTERM, Arc::clone(&should_stop))?;

    let mut child = Command::new("/bin/bash")
        .arg("-i")
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Could not start child");

    let stdin = io::stdin();

    while !should_stop.load(Ordering::Relaxed) {
        let mut buffer = String::new();
        match stdin.read_line(&mut buffer) {
            Ok(size) => {
                if size > 0 {
                    if is_special_command(&buffer) {
                        println!("Got special command!")
                    } else {
                        let child_stdin = child.stdin.as_mut();
                        if let Some(out) = child_stdin {
                            match out.write(buffer.as_bytes()) {
                                Ok(_size) => (),
                                Err(_) => ()
                            }
                        }

                    }
                }
            }
            Err(_) => ()
        }
    }
    println!("Got CTRL+C. Exiting term wrap");
    child.kill()?;
    Ok(())
}
