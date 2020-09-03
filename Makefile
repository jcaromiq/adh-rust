VERSION=$(version)
RELEASE_NOTES ?=$(version)
BRANCH=master
FILENAME_MAC=./target/release/adh-$(VERSION)-x86_64-apple-darwin.tar.gz
FILENAME_LINUX=./target/release/adh-$(VERSION)-x86_64-linux.tar.gz
hub_exec ?= hub

build-release:
	cargo build --release

upload-mac: build-release tar-release-mac
	$(hub_exec) release create -a $(FILENAME_MAC) -m $(RELEASE_NOTES) -t $(BRANCH) $(VERSION)

upload-linux: build-release tar-release-linux
	$(hub_exec) release create -a $(FILENAME_LINUX) -m $(RELEASE_NOTES) -t $(BRANCH) $(VERSION)

tar-release-mac: build-release
	tar -czvf $(FILENAME_MAC) ./target/release/adh

tar-release-linux: build-release
	tar -czvf $(FILENAME_LINUX) ./target/release/adh
