build-integer:
	@cargo +nightly build --no-default-features --features=integer
	
build:
	@cargo +nightly build --no-default-features
	
run-tests:
	@cargo +nightly test --no-default-features --features=integer

clippy:
	@cargo +nightly clippy --no-default-features --features=integer
