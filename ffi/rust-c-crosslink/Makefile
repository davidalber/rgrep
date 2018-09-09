LIBS = -L target/debug -lrust_c_avro

.PHONY: all rustlib clean

all: it

it: it.o rustlib
	cc -o it it.o $(LIBS)

it.o: it.c
	cc -c it.c

rustlib:
	cargo build

clean:
	rm -f it it.o

cleanall: clean
	cargo clean
