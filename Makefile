# paths
PREFIX = /usr/local
BINPATH = $(PREFIX)/bin

build:
	@cargo build --release --color never

install: build
	@cp target/release/msel $(DESTDIR)$(BINPATH)/msel
	@echo installed!

uninstall: build
	@rm -r $(DESTDIR)$(BINPATH)/msel

clean:
	@rm -rf target/release
	@echo cleaned!

.PHONY: install clean build
