# counter

### Usage
Start fuel node
```bash
fuel-core --db-type in-memory
```

Deploy contract
```bash
cd programs/counter
forc deploy --locked
```

Set deployed contract ID
```bash
echo "CONTRACT_ID="0x123...." > programs/counter-rs/.env
```

Run the backend
```bash
RUST_LOG=debug cargo run --bin server
```

Make a sample request to the backend
```bash
curl -X GET http://localhost:8080/counter
```
