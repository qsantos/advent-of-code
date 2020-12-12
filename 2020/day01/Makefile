CFLAGS+=-Wall -Wextra -Wpedantic -Wshadow -Wvla -Wconversion -O3
CXXFLAGS+=-Wall -Wextra -Wpedantic -Wshadow -Wvla -Wconversion -O3 -std=c++17
all: main-c main-cpp main-rs
main-c:
main-cpp:
main-rs: main-rs.rs
	rustc "$^" -o "$@"
