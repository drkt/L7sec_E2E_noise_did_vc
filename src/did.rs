
use anyhow::Result;
use ssi::jwk::{JWK, Params};
use ssi::did::DIDMethod;
use ssi::did::example::ExampleDID;

pub async fn create_did_and_key() -> Result<(String, JWK)> {
    let jwk = JWK::generate_ed25519()?;
    let method = ExampleDID;
    let did = method.generate(&jwk).await?;
    Ok((did, jwk))
}
