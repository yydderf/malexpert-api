# API for MalwareExpert

### Quick Start

Configuration can be adjusted by modifying `Rocket.toml`

```bash
MALEXP_HMAC_KEY=$(openssl rand -base64 32) \
cargo run
# or toggle the debugging message
MALEXP_HMAC_KEY=$(openssl rand -base64 32) \
RUST_LOG=malexpert_api=debug \
cargo run
```

### Docker

```bash
docker build -t malexpert-api .
docker run -it --rm --name malexpert-api -p 8000:8000 malexpert-api:latest
```
