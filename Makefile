build:
	cargo build --release
	cp man-pages/blokator man-pages/blokator.1
	gzip man-pages/blokator.1 -qf

build-debug:
	cargo build

build-android:
	cargo build --features android

install:
	install -Dm755 target/release/blokator /usr/bin/blokator
	install -Dm644 man-pages/blokator.1.gz /usr/share/man/man1/blokator.1.gz

clean:
	cargo clean