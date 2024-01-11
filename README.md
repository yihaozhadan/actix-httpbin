# actix-httpbin

httpbin.org implementation using Rust Actix framework. Learning Rust, Actix framework by implementing https://httpbin.org APIs.

## Get Started

```sh
cargo run
```

Run with Docker

```sh
docker build . -t actix-httpbin:0.2.0
```

```sh
docker run --name actix-httpbin -p 8080:8080 actix-httpbin:0.2.0
```
