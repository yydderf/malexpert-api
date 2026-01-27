# API for MalwareExpert

### Quick Start

Configuration can be adjusted by modifying `Rocket.toml`

```bash
cargo run
```

### Docker

```bash
docker build -t malexpert-api .
docker run -it --rm --name malexpert-api -p 8000:8000 malexpert-api:latest
```
