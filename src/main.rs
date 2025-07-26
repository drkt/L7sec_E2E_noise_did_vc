
use anyhow::Result;
use noise_did_vc::did::create_did_and_key;
use noise_did_vc::vc::{issue_vc, verify_vc};
use noise_did_vc::logging::log_event;
use noise_did_vc::noise::run_noise_handshake;
use log::info;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // Beispiel: Erzeuge DIDs für Alice und Bob
    let (alice_did, alice_jwk) = create_did_and_key().await?;
    let (bob_did, bob_jwk) = create_did_and_key().await?;

    info!("Alice DID: {}", alice_did);
    info!("Bob DID: {}", bob_did);

    // Handshake simulieren (Dummy)
    run_noise_handshake(&alice_jwk, &bob_jwk)?;

    // VC erzeugen & validieren
    let vc = issue_vc(&alice_did, &bob_did, &alice_jwk).await?;
    verify_vc(&vc, &alice_did).await?;

    Ok(())
}
