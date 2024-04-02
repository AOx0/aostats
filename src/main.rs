use bstr::ByteSlice;
use std::process::Command;

fn main() {
    let res = Command::new("neofetch").arg("--stdout").output();
    match res {
        Ok(command) => {
            let stdout = command.stdout;

            println!("{}", stdout.trim().as_bstr());
        }
        Err(err) => {
            println!("neofetch: {}", err);
        }
    }

    let res = Command::new("acpi").output();
    match res {
        Ok(command) => {
            display_battery(command);
        }
        Err(err) => {
            println!("acpi: {}", err);
        }
    }
}

fn display_battery(command: std::process::Output) {
    let stdout = command.stdout;

    for line in stdout.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let Some((_bat, status)) = line.split_once_str(": ") else {
            continue;
        };

        print!(
            "Battery: {}",
            if status.contains_str("Not") {
                "N"
            } else if status.contains_str("Dis") {
                "D"
            } else {
                "C"
            }
        );

        for word in status.trim().split_str(" ") {
            if word.contains_str("%") {
                print!("{} ", word.trim().trim_end_with(|c| c == ',').as_bstr());
            } else if word.contains_str(":") {
                print!("[{}]", word.trim().as_bstr());
            }
        }

        println!();
    }
}
