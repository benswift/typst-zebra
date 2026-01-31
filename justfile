build:
    cd plugin && cargo build --release --target wasm32-unknown-unknown
    wasm-opt --enable-bulk-memory-opt -Oz -o src/zebra.wasm plugin/target/wasm32-unknown-unknown/release/typst_zebra.wasm
    typst compile --root . examples/comparison.typ -f svg
    typst compile --root . examples/example.typ -f svg
    typst compile --root . examples/tests.typ -f svg
