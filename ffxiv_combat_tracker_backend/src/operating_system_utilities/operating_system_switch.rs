use super::linux_memory::memory_ops;
use std::env;

pub fn detect_operating_system() {
    let operating_system: &str = env::consts::OS;
    match operating_system {
        "linux" => memory_ops::linux_memory_parser(),
        "windows" => println!("Windows!"),
        "macos" => println!("MacOS!"),
        _ => todo!(),
    }
}
