srcs := $(shell find src/ -name "*.rs")
bins := $(patsubst src/%.rs, bin/%, $(srcs))

.PHONY: all clean

all: $(bins)

bin:
	mkdir bin

bin/%: src/%.rs | bin
	lrsc "$<" -o "$@"

clean:
	rm bin/*
