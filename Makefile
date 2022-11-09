build:
	cargo build --release
	
build-debug:
	cargo build

build-android:
	cargo build --features android

install:
	cp target/release/blokator /usr/bin/blokator

clean:
	cargo clean