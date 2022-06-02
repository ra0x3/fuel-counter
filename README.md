# counter

## Usage

### OSX

Start Fuel node, deploy contract, and start API webserver

```bash
RUST_LOG=debug RUSTFLAGS="-Clink-arg=-Wl" cargo run --target x86_64-apple-darwin
```

### Ubuntu

```bash
RUST_LOG=debug RUSTFLAGS="-Clink-arg=-Wl,--allow-multiple-definition" cargo run
--target x86_64-unknown-linux-gnu
```

### Interaction

Make a request to the contract via the API server

```bash
curl -X POST http://127.0.0.1:8080/count
```
