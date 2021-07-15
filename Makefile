build-integer:
	@cargo +nightly build --no-default-features --features=integer

build:
	@cargo +nightly build --no-default-features

run-tests:
	@cargo +nightly test --features=integer

clippy:
	@cargo +nightly clippy --features=integer