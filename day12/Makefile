CFLAGS+=-O3 -Wall -Wextra -Wpedantic -Wconversion -Woverflow -Wshadow -Wvla -std=c99 -Wstrict-prototypes -g
LDFLAGS+=-O3
LDLIBS:=

main: main.o

clean:
	rm -f *.o *.d

destroy: clean
	rm -f main

.PHONY: clean destroy
