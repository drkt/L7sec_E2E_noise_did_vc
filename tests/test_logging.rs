// tests/test_logging.rs

use std::fs;
use std::path::Path;

use noise_did_vc::logging::log_event;

#[test]
fn test_log_writes_to_file() {
    let log_path = "audit.log";

    // Vorher: Alte Log-Datei ggf. entfernen
    if Path::new(log_path).exists() {
        fs::remove_file(log_path).expect("Failed to delete old log file");
    }

    // Event schreiben
    log_event("Test log entry");

    // Datei muss nun existieren
    assert!(Path::new(log_path).exists(), "Log file not created");

    // Inhalt prüfen
    let contents = fs::read_to_string(log_path).expect("Failed to read log file");
    assert!(
        contents.contains("Test log entry"),
        "Log entry not found in file"
    );
}
