# Noise + DID + VC Demo

## Requirements

- Rust (>=1.70)
- Docker (optional)
- Internet for pulling crates

## Build Locally

```bash
cargo build
```

## Run Alice

```bash
ROLE=alice cargo run
```

## Run Bob

```bash
ROLE=bob cargo run
```

## Test Logging

```bash
cargo test --test test_log
```

## Run in Docker

```bash
docker build -t noise_did_vc .
docker run --rm -e ROLE=alice noise_did_vc
```

## Notes

- Uses `snow` for Noise_XX handshake
- Uses `did-key` crate for simple DIDs
- Verifiable Credential flow is simplified