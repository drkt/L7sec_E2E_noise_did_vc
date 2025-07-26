
use log::info;

pub fn log_event(message: &str) {
    info!(target: "audit", "{}", message);
}
