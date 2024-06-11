#![allow(unused_imports)]

// Run the <windows_container> module independently:
// cargo run -p windows_container

// Build the <windows_container> module independently:
// cargo build -p windows_container

// Command:
// docker run -it --rm -p 8006:8006 --device=/dev/kvm --cap-add NET_ADMIN --stop-timeout 120 --name windows dockurr/windows

use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use std::process::{exit, Command};

fn main() {
    if !check_os() {
        println!("This script only supports Linux operating systems.");
        exit(1);
    }

    if !check_kvm_support() {
        println!("KVM is not supported on this system.");
        exit(1);
    }

    let image_name = "dockurr/windows";

    if !validate_input(image_name) {
        println!("Invalid Docker image name.");
        exit(1);
    }

    run_docker_command(image_name);
}

fn check_os() -> bool {
    env::consts::OS == "linux"
}

fn check_kvm_support() -> bool {
    Path::new("/dev/kvm").exists()
}

fn validate_input(image_name: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9\-_/]+$").unwrap();
    re.is_match(image_name)
}

fn run_docker_command(image_name: &str) {
    let status = Command::new("docker")
        .args([
            "run",
            "-it",
            "--rm",
            "-p",
            "8006:8006",
            "--device=/dev/kvm",
            "--cap-add",
            "NET_ADMIN",
            "--stop-timeout",
            "120",
            "--name",
            "windows",
            image_name,
        ])
        .status()
        .expect("Failed to execute Docker command");

    if !status.success() {
        println!("Error running Docker command: {:?}", status);
        exit(1);
    }
}
