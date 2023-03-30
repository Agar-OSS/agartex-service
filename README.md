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
You can change environment variables by modifying .env

```
cargo run
```

## Docker

```
docker build -t agaross.azurecr.io/agar-oss/latex-base latex
docker build -t agaross.azurecr.io/agar-oss/agartex-service .
```
