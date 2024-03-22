# Call From Shared Library

This is an example of how to load a dynamic library
and call a function from it.

Compile library and executable, put them together and run
```shell
mkdir ./demo
cargo clean
cargo build --release --all
cp ./target/release/call-from-shared ./demo/
cp ./target/release/deps/libshared.so ./demo/
cd ./demo
./call-from-shared
cd ..
cargo clean
rm -r ./demo
```

