svgo-config := '''
export default {
  plugins: [
    { name: "removeAttrs", params: { attrs: "class" } },
    "preset-default",
    "removeViewBox",
  ],
  js2svg: { indent: 2, pretty: true },
}
'''

default: fmt build test

build:
    cd plugin && cargo build --release --target wasm32-unknown-unknown
    wasm-opt --enable-bulk-memory-opt -Oz -o src/zebra.wasm plugin/target/wasm32-unknown-unknown/release/typst_zebra.wasm

    typst compile --root . examples/readme-header.typ -f svg
    typst compile --root . examples/readme-optimized-path.typ -f svg
    @sed -i '' '1s/\([0-9]\)pt"/\1"/g' examples/readme-*.svg
    @echo '{{svgo-config}}' > svgo.config.js
    svgo examples/readme-*.svg
    @rm svgo.config.js

fmt:
    cd plugin && cargo fmt
    typstyle --line-width 100 -i src examples

test:
    cd plugin && cargo test
    typst compile --root . examples/tests.typ -f svg
    typst compile --root . examples/test-readme.typ -f svg

alias w := watch
watch typst-file="tests":
    typst watch --root . '{{ if typst-file =~ "\\.typ$" { typst-file } else { "examples/" + typst-file + ".typ" } }}' -f svg
