all: release


build:
	@cargo build
	cp target/debug/juicyj joosc

clean:
	@cargo clean
	rm -rf joosc juicyj.zip **/*.rs.bk

release:
	@cargo build --release
	cp target/release/juicyj joosc

zip:
	zip juicyj.zip Makefile
	zip juicyj.zip Cargo.*
	zip -r juicyj.zip src
