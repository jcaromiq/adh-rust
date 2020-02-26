VERSION=$(version)
RELEASE_NOTES ?=$(version)
BRANCH=master
FILENAME=./target/release/adh-$(VERSION)-x86_64-apple-darwin.tar.gz
hub_exec ?= hub

build-release:
	cargo build --release

upload: build-release tar-release
	$(hub_exec) release create -a $(FILENAME) -m $(RELEASE_NOTES) -t $(BRANCH) $(VERSION)

tar-release: build-release
	tar -czvf $(FILENAME) ./target/release/adh
