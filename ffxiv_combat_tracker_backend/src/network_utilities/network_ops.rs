use std::process::{Command, Output};

pub async fn get_routing_table() -> Option<String> {
    let route_command_output: Output = Command::new("route")
        .output()
        .expect("Failed to acquire routing table.");
    let output: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&route_command_output.stdout);
    let default_routes: Vec<&str> = output
        .lines()
        .filter(|&line| line.to_lowercase().starts_with("default"))
        .collect();
    let mut lowest_metric = u32::MAX;
    let mut default_iface = None;
    for line in default_routes {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.get(2)? == &"0.0.0.0" {
            if let Some(metric) = parts.get(4)?.parse::<u32>().ok() {
                if metric < lowest_metric {
                    lowest_metric = metric;
                    default_iface = parts.get(7).cloned();
                }
            }
        }
    }
    return default_iface.map(String::from);
}
