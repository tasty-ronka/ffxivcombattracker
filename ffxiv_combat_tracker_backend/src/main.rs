mod operating_system_utilities;
use operating_system_utilities::operating_system_switch;
#[tokio::main]
async fn main() {
    operating_system_switch::detect_operating_system().await;
}
