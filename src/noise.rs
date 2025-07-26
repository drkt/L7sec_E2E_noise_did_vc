use snow::{Builder, Keypair, params::NoiseParams};
use std::sync::Arc;

pub fn build_handshake(static_key: Option<Keypair>) -> anyhow::Result<Builder<'static>> {
    let noise_params: NoiseParams = "Noise_XX_25519_AESGCM_SHA256".parse()?;
    let builder = Builder::new(noise_params);

    let builder = match static_key {
        Some(kp) => {
            let private = Arc::new(kp.private.clone());
            builder.local_private_key(&private[..])
        },
        None => {
            let keypair = builder.generate_keypair()?;
            let private = Arc::new(keypair.private.clone());
            builder.local_private_key(&private[..])
        },
    };

    Ok(builder)
}