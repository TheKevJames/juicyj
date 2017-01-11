all: release


build:
	@cargo build
	cp target/debug/juicyj joosc

clean:
	@cargo clean
	rm -rf joosc **/*.rs.bk

release:
	@cargo build --release
	cp target/release/juicyj joosc
