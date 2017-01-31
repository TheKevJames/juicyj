all: release


build:
	@cargo build
	cp target/debug/juicyj joosc

clean:
	@cargo clean
	rm -rf joosc
	rm -rf juicyj.zip
	rm -rf **/*.rs.bk
	rm -rf grammar/jlalr/*.class

release:
	@cargo build --release
	cp target/release/juicyj joosc

grammar: grammar/joos.lr1
grammar/joos.cfg: grammar/joos.grammar grammar/cfgerizer/cfgerize.py
	./grammar/cfgerizer/cfgerize.py $< $@
grammar/joos.lr1: grammar/joos.cfg grammar/jlalr/Jlr1.class
	cd grammar && cat joos.cfg | java jlalr.Jlr1 > joos.lr1
grammar/jlalr/Jlr1.class: grammar/jlalr/Jlalr1.java
	cd grammar && javac jlalr/Jlalr1.java

zip:
	@rm -rf juicyj.zip
	zip juicyj.zip Makefile
	zip juicyj.zip Cargo.*
	zip -r juicyj.zip grammar/joos.lr1
	zip -r juicyj.zip src
