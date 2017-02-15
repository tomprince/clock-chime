.PHONY = build

VERSION = $(shell cargo metadata --no-deps | jq -r '.packages[0].version')

ARCHIVE = clock-chime-$(VERSION).tar.gz
RPM = x86_64/clock-chime-$(VERSION)-1.fc26.x86_64.rpm

build: $(RPM)

$(RPM): $(ARCHIVE)
	fedpkg local

$(ARCHIVE): .git/refs/tags/v$(VERSION)
	git archive --prefix clock-chime-$(VERSION)/ -o $(ARCHIVE) v$(VERSION) 
