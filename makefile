build:
	cargo build --release
run:
	cargo build --release && sudo ./target/release/socker config.json   
