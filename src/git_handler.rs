use std::process::Command;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub(crate) fn send_message(message: &String) -> String {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("chat.txt")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", message) {
        eprintln!("Couldn't write to file: {}", e);
    }

    let command_output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "git add . && git commit -m \"message\" && git push"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("git add . && git commit -m \"message\" && git push")
            .output()
            .expect("failed to execute process")
    };

    String::from_utf8(command_output.stdout).unwrap()
}