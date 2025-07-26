use noise_did_vc::logging::{log_event, AuditEvent};

#[test]
fn test_logging() {
    let did = "did:example:alice";
    log_event(AuditEvent::DidResolved { did: did.to_string() });
}