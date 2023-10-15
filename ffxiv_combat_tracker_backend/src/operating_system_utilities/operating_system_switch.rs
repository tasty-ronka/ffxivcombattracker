use crate::operating_system_utilities::memory_ops;

pub async fn detect_operating_system() {
    memory_ops::memory_parser().await
}
