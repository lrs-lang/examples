srcs := $(shell find src/ -name "*.rs")
bins := $(patsubst src/%.rs, bin/%, $(srcs))

.PHONY: all clean

-include config.mk

all: $(bins)

bin:
	mkdir bin

bin/%: src/%.rs | bin
	lrsc $(RFLAGS) "$<" -o "$@"

clean:
	rm bin/*
