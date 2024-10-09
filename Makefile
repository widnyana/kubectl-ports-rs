VERSION  := latest

default: run

## Builds the app for current os-arch
build:
	cargo build --release

## Runs the app
run:
	@CARGO_INCREMENTAL=1 cargo +nightly fmt && make lint && cargo run

## Run clippy
lint:
	@find . -type f | grep '\/src\/.*\.rs'| xargs touch && cargo clippy --all-targets --workspace

## Fix lint
lint-fix:
	@cargo fix


fmt-dep:
	@rustup toolchain install nightly-x86_64-unknown-linux-gnu
	@rustup component add --toolchain nightly-x86_64-unknown-linux-gnu rustfmt


## Run format
fmt: fmt-dep
	@cargo +nightly fmt

## Analyse for unsafe usage - `cargo install cargo-geiger`
analyse:
	@cargo geiger

## Install release binary to $HOME/.local/bin
install:
	@mkdir $$HOME/.local/bin -p
	@cp ./target/release/kubectl-ports $$HOME/.local/bin

clean:
	@cargo clean

## Release tag
release:
	@git tag -a ${V} -m "Release ${V}" && git push origin ${V}

## Delete tag
delete-tag:
	@git tag -d ${V} && git push --delete origin ${V}
