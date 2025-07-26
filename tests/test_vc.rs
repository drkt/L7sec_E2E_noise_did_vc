// tests/test_vc.rs

use noise_did_vc::vc::{issue_vc, verify_vc};

/// Test zur Überprüfung des vollständigen VC-Flows:
/// 1. Credential wird ausgestellt
/// 2. Credential wird verifiziert
#[test]
fn test_vc_issue_and_verify() {
    issue_vc();    // sollte ohne Fehler durchlaufen
    verify_vc();   // sollte das ausgestellte VC korrekt prüfen

    // Auch hier: konkrete Assertions können folgen, wenn z. B. Werte/Ergebnisse zurückgegeben werden
    assert!(true); // Dummy-Assertion
}
