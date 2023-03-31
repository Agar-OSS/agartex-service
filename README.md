# agartex-service

Service with backend functionalities for AgarTeX

## How to run postgres from project root
This requires that postgres is not running on your machine already.

If it is, change the port mapping.
```
cd postgres && docker compose up -d
```

## How to run service
This requires postgres to be running.
```
cargo run
```

Run linter
```
cargo clippy --all-targets --all-features -- -D warnings
```

## Docker

### Build
```
docker build -t agaross.azurecr.io/agar-oss/latex-base latex
docker build -t agaross.azurecr.io/agar-oss/agartex-service .
```

### Run
```
docker-compose up -d # To start database and service
docker-compose down -v # To stop database and service
```
