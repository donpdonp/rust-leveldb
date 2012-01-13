DUMMY := dummy.so
all: ${DUMMY} main

test_suites := ./test/test.rs

main: dummy.so
	rustc -L . src/leveldb.rs -o main

dummy.so: src/leveldb.rs
	rustc --lib src/leveldb.rs -o dummy.so
	touch dummy.so

# FIXME: proper dependencies
check: all
	rustc -L . --test test/test.rs
	./test/test

clean:
	rm -f dummy.so libleveldb-*.so
	rm -rf test/test
	rm -f main

.PHONY: all clean
