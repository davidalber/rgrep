# Building
## Via `make`
- To build the executable use `make`.
- To clean the executable and C artifacts use `make clean`.
- To clean everything use `make cleanall`.

## (Alternative) Build Directly
Build the Rust library:
```
$ cargo build
```

Build the executable:
```
$ gcc it.c -L $PWD/target/debug -lrust_c_avro -o it
```

# Running the Executable
```
$ ./it
```
