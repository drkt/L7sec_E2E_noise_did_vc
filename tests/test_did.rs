// tests/test_did.rs

use noise_did_vc::did::create_did_and_key;

/// Einfacher Test zur Validierung der DID-Erzeugung.
/// Hier wird nur geprüft, dass die Funktion ohne Fehler durchläuft.
/// Eine detaillierte Überprüfung (z. B. Format der DID) kann bei Bedarf ergänzt werden.
#[test]
fn test_create_did_and_key() {
    create_did_and_key();
    // Hier könnten weitere Assertions folgen, z. B. auf erzeugte Dateien oder Ausgaben
    assert!(true); // Dummy-Assertion für Testframework
}
