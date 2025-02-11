PREFIX ?= /usr
SYSCONFDIR ?= /etc
DATADIR ?= $(PREFIX)/share
DESTDIR ?=
MY_ENGINE_DATA_DIR ?= $(DATADIR)/ibus-test/

MKFILE_PATH := $(abspath $(lastword $(MAKEFILE_LIST)))
MKFILE_DIR := $(dir $(MKFILE_PATH))

all: ibus-test.xml config.h ../target/release/ibus-test


config.h: config.h.in
	sed \
	    -e "s:@DATADIR@:$(DATADIR)/:g" $< > $@


ibus-test.xml: ibus-test.xml.in
	sed \
	    -e "s:@BINARY@:$(PREFIX)/bin/ibus-test:g" \
	    -e "s:@DATADIR@:$(DATADIR)/:g" $< > $@

# ../target/release/ibus-test: