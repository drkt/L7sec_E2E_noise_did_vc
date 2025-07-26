
# noise_did_vc (using `ssi` crate)

This project demonstrates a secure communication prototype using:
- 🎯 DID key generation
- 📄 Verifiable Credential issuance and verification
- 🔐 Simulated Noise handshake (placeholder)
- 🧾 Audit logging
- 🧰 Modular architecture using Rust and async runtime

## 🔧 Requirements

- Rust (>= 1.70)
- Cargo
- Docker (optional, for container builds)

## ▶️ Run

```bash
cargo run
```

You should see:
- Two DIDs (Alice and Bob) generated
- A Verifiable Credential issued by Alice to Bob
- Simulated handshake using Noise placeholder

## ✅ Test

This version includes VC verification inline. You can extend integration tests under `tests/`.

## 📁 Structure

- `src/main.rs` – entry point
- `did.rs` – DID creation via `ssi`
- `vc.rs` – VC creation and verification
- `noise.rs` – placeholder Noise handshake
- `logging.rs` – structured log support

## 🐳 Docker (optional)

To build in a container:

```bash
docker build -t noise_did_vc .
```

## 📌 Notes

- The Noise handshake is currently mocked.
- You can replace it using the `snow` crate for full integration.

## 📚 Resources

- https://github.com/spruceid/ssi
- https://crates.io/crates/snow

---

© 2025 IETF ATLS Prototype
