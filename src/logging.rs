#[derive(Debug)]
pub enum AuditEvent {
    DidResolved { did: String },
    VcReceived { issuer: String },
    Error { message: String },
}

pub fn log_event(event: AuditEvent) {
    match event {
        AuditEvent::DidResolved { did } => log::info!("Resolved DID: {}", did),
        AuditEvent::VcReceived { issuer } => log::info!("Received VC from: {}", issuer),
        AuditEvent::Error { message } => log::error!("Error: {}", message),
    }
}