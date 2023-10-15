mod network_utilities;
mod operating_system_utilities;
// use operating_system_utilities::operating_system_switch;
use network_utilities::network_ops::get_routing_table;
#[tokio::main]
pub async fn main() {
    // operating_system_switch::detect_operating_system().await;
    match get_routing_table().await {
        Some(iface) => println!("Default interface {}", iface),
        None => println!("Unable to determine interface"),
    }
}
