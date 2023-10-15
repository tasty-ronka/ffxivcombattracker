extern crate sysinfo;
extern crate tokio;
use std::collections::HashMap;
use std::time::Duration;
use sysinfo::{Pid, Process, ProcessExt, System, SystemExt};
use tokio::time::sleep;

pub async fn memory_parser() {
    let mut _sys: System = System::new_all();

    loop {
        _sys.refresh_all();

        let mut xiv_processes: HashMap<&Pid, &Process> = HashMap::new();

        for (pid, process) in _sys.processes() {
            if process.name().to_uppercase().contains("XIV") {
                xiv_processes.insert(pid, process);
            }
        }

        println!(
            "Process HashMap: {:#?}",
            reduce_process_hashmap(&mut xiv_processes)
        );
        // let mut sorted_processes: Vec<_> = xiv_processes.iter().collect();
        // sorted_processes
        //     .sort_by(|a_process, b_process| a_process.memory().cmp(&b_process.memory()));

        // if let Some(&highest_mem_process) = sorted_processes.last() {
        //     println!(
        //         "Process: {}, Memory: {} mb",
        //         highest_mem_process.name(),
        //         convert_bytes(highest_mem_process.memory())
        //     );
        sleep(Duration::from_secs(10)).await;
    }
}
// }

fn convert_bytes(_bytes: u64) -> f64 {
    (_bytes as f64 / 1_048_576.0).round()
}

fn reduce_process_hashmap(_hashmap: &mut HashMap<&Pid, &Process>) -> &mut HashMap<&Pid, &Process> {
    if let Some((&pid, _)) = _hashmap.iter().max_by_key(|(_, &process)| process.memory()) {
        let highest_mem_process = _hashmap.remove(&pid).unwrap();
        _hashmap.clear();
        _hashmap.insert(&pid, highest_mem_process);
    }
    return _hashmap;
}
