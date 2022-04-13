docker run --rm -it -v %CD%:/app -w /app ekidd/rust-musl-builder cargo build --release
docker build -t k8s-gr4/app-rust --rm .
