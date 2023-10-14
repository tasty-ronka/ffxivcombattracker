extern crate sysinfo;
use sysinfo::{ProcessExt, System, SystemExt};

pub fn linux_memory_parser() {
    let mut _sys: System = System::new_all();

    _sys.refresh_all();

    for (pid, process) in _sys.processes() {
        // println!("{} {:?}", pid, process);
        if process.name().to_uppercase().contains("XIV") {
            println!("Found FFXIV with PID: {}", pid)
        }
    }
}
