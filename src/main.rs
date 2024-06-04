use bstr::ByteSlice;
use clap::Parser;
use std::{ops::Not, process::Command};

#[derive(Parser)]
struct Args;

fn main() {
    let Args: Args = Args::parse();
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

    let disks = sysinfo::Disks::new_with_refreshed_list();
    for disk in &disks {
        let used = disk.total_space() - disk.available_space();
        println!(
            "Disk: \"{}\" {}GiB / {}GiB [{:.1} %]",
            disk.mount_point().display(),
            used / 1_073_741_824,
            disk.total_space() / 1_073_741_824,
            (used as f64 / disk.total_space() as f64) * 100.,
        );
    }

    let res = Command::new("ip").arg("a").output();
    match res {
        Ok(command) => {
            let stdout = command.stdout;
            let mut last_iname = None;
            for line in stdout.lines() {
                if !line.starts_with(b" ") {
                    let mut words = line.split_str(b" ");
                    let iname = words.nth(1).unwrap().trim_end_with(|c| c == ':');
                    if !iname.starts_with(b"lo") {
                        last_iname = Some(iname);
                    }
                } else if let (Some(iname), true) =
                    (last_iname, line.trim_start().starts_with(b"inet"))
                {
                    let line = line.trim_start();
                    let mut words = line.split_str(b" ");
                    let addr = words.nth(1).unwrap();

                    if line.contains_str(b"::").not() && line.contains_str(b":").not() {
                        println!(
                            "Inet: [{iname}] {address}",
                            address = addr.as_bstr(),
                            iname = iname.as_bstr()
                        )
                    }
                }
            }
        }
        Err(err) => {
            println!("ip: {}", err);
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
