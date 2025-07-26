mod noise;
mod did;
mod vc;
mod logging;

use logging::{log_event, AuditEvent};

fn main() {
    env_logger::init();
    let role = std::env::var("ROLE").unwrap_or("alice".to_string());
    match role.as_str() {
        "alice" => println!("Running Alice"),
        "bob" => println!("Running Bob"),
        _ => eprintln!("Unknown ROLE"),
    }
}