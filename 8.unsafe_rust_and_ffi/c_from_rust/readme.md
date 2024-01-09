## Build C Library

`clang -c adder/adder.c -o adder/adder.o`

`llvm-lib /out:./adder/adder.lib ./adder/adder.o`

## Run

`RUSTFLAGS='-L ./adder' cargo run`
