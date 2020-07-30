.PHONY: run-client
run-client: a.out use librust_test.so libc_test.so
	cargo build
	LD_LIBRARY_PATH=. ./a.out
	LD_LIBRARY_PATH=. ./use

.PHONY: run-server
run-server: server librust_test.so libc_test.so
	cargo build
	LD_LIBRARY_PATH=. ./server

server: ../seamless-server/src/main.rs
	cargo build
	cp ../target/debug/seamless-server server

libseamless.so: ../seamless-dylib/src/lib.rs
	cargo build
	cp ../target/debug/libseamless_dylib.so libseamless.so

a.out: use.c libseamless.so
	gcc use.c -L. -lseamless

use: use.rs libseamless.so
	rustc use.rs -L. -lseamless

librust_test.so: rust_test.rs
	rustc rust_test.rs

libc_test.so: c_test.c
	gcc -shared -fPIC -o libc_test.so c_test.c

.PHONY: clean
clean:
	-rm a.out
	-rm use
	-rm libseamless.so
	-rm libc_test.so
	-rm librust_test.so
	-rm server
	cargo clean