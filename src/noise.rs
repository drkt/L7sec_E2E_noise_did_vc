
use anyhow::Result;
use ssi::jwk::JWK;

pub fn run_noise_handshake(_local_key: &JWK, _remote_key: &JWK) -> Result<()> {
    println!("Simulated Noise handshake with key material.");
    Ok(())
}
