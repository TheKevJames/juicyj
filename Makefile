all: release


build:
	@cargo build
	cp target/debug/juicyj joosc

clean:
	@cargo clean
	rm -rf joosc
	rm -rf juicyj.zip
	rm -rf **/*.rs.bk
	rm -rf docs/*.pdf
	rm -rf grammar/jlalr/*.class

docs: docs/a1.pdf docs/a4.pdf docs/a5.pdf
docs/a1.pdf: docs/a1.md
docs/a4.pdf: docs/a4.md
docs/a5.pdf: docs/a5.md

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

ASMSOURCES := $(shell find output -name '*.s')
ASMOBJECTS := $(ASMSOURCES:%.s=%.o)

stdlib/runtime.o: stdlib/runtime.s
main: $(ASMOBJECTS) stdlib/runtime.o
	ld -melf_i386 -o $@ $^


.SUFFIXES:
.SUFFIXES: .md .o .pdf .s

.md.pdf:
	pandoc -s $^ -o $@
output/%.o: output/%.s
	nasm -O1 -f elf -g -F dwarf $^
stdlib/%.o: stdlib/%.s
	nasm -O1 -f elf -g -F dwarf $^
