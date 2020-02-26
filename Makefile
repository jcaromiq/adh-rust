VERSION=$(version)
RELEASE_NOTES=$(notes)
BRANCH=master
FILENAME=./target/release/adh-$(VERSION)-x86_64-apple-darwin.tar.gz

build-release:
	cargo build --release

upload: build-release tar-release
	hub release create -a $(FILENAME) -m $(RELEASE_NOTES) -t $(BRANCH) $(VERSION)

tar-release: build-release
	tar -czvf $(FILENAME) ./target/release/adh
