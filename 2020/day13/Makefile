CFLAGS+=-Wall -Wextra -Wpedantic -Wshadow -Wvla -Wconversion -O3 -std=c99
CXXFLAGS+=-Wall -Wextra -Wpedantic -Wshadow -Wvla -Wconversion -O3 -std=c++17
all: main-cpp main-rs
main-cpp:
main-rs: main-rs.rs
	rustc "$^" -o "$@"
