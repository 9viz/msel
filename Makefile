# paths
PREFIX = /usr/local
BINPATH = $(PREFIX)/bin

build:
	@cargo build --release --color never

install: build
	@cp ./target/debug/msel $(DESTDIR)$(BINPATH)/msel
	@echo installed!

clean:
	@rm -rf ./target/release
	@echo cleaned!

.PHONY: install clean build
