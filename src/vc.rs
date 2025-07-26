
use anyhow::Result;
use ssi::vc::{Credential, Issuer, LinkedDataProofOptions, URI, VerifiableCredential};
use ssi::ldp::ProofSuite;
use ssi::jwk::JWK;
use ssi::vc::check::CredentialValidator;

pub async fn issue_vc(issuer_did: &str, subject_did: &str, jwk: &JWK) -> Result<Credential> {
    let mut credential = Credential::default();
    credential.issuer = Some(Issuer::URI(URI::String(issuer_did.to_string())));
    credential.credential_subject = Some(serde_json::json!({
        "id": subject_did,
        "claim": "example"
    }));

    let proof = credential.generate_proof(jwk, &LinkedDataProofOptions::default()).await?;
    credential.add_proof(proof);
    Ok(credential)
}

pub async fn verify_vc(vc: &Credential, expected_issuer: &str) -> Result<()> {
    let result = vc.verify(Some(&LinkedDataProofOptions::default()), &ssi::did_resolve::DIDResolver::default()).await;
    assert!(result.errors.is_empty(), "Verification failed: {:?}", result);
    Ok(())
}
