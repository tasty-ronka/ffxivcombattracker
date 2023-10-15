extern crate sysinfo;
extern crate tokio;
use std::collections::HashMap;
use std::time::Duration;
use sysinfo::{Pid, Process, ProcessExt, System, SystemExt};
use tokio::time::sleep;

// Returns the process matching "XIV" in the name with the highest memory usage; the
// return is a HashMap with the process PID and the process itself.

pub async fn memory_parser() {
    let mut _sys: System = System::new_all();

    loop {
        _sys.refresh_all();

        let mut xiv_processes: HashMap<&Pid, &Process> = HashMap::new();

        for (pid, process) in _sys.processes() {
            if process.name().to_uppercase().contains("XIV") {
                xiv_processes.insert(&pid, &process);
            }
        }

        xiv_processes = reduce_process_hashmap(xiv_processes);
        println!("{:?}\n\n\n", xiv_processes);

        sleep(Duration::from_secs(10)).await;
    }
}

fn reduce_process_hashmap<'a>(_hashmap: HashMap<&'a Pid, &'a Process>) -> HashMap<&Pid, &Process> {
    let mut new_hashmap: HashMap<&Pid, &Process> = HashMap::new();

    if let Some((&pid, _)) = _hashmap.iter().max_by_key(|(_, &process)| process.memory()) {
        new_hashmap.insert(&pid, _hashmap[&pid]);
    }
    return new_hashmap;
}
