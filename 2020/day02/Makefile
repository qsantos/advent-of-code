CFLAGS+=-Wall -Wextra -Wpedantic -Wshadow -Wvla -Wconversion -O3
all: main-c main-rs
main-c:
main-rs: main-rs.rs
	cargo build
	mv target/debug/"$@" .
