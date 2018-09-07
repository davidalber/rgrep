Build the Rust library:
```
$ cargo build
```

Build the executable:
```
$ gcc it.c -L $PWD/target/debug -lrust_c_avro -o it
```

Run the executable:
```
$ ./it
```
