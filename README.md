# WebAssembly for Proxies (Rust SDK) - 3scale version

This is a fork of the upstream [proxy-wasm-rust-sdk](https://github.com/proxy-wasm/proxy-wasm-rust-sdk) repository intended to be used as the base proxy-wasm
library to develop extensions in Rust at Red Hat.

Most patches you will find here have associated open pull requests or issues
in the upstream project from various people.

3scale intends for this fork to closely follow upstream while addressing
shortcomings we believe to be important for WASM module development. Hopefully
we'll keep differences with upstream to a minimum as new upstream releases
address these issues. This means we expect some such issues to be solved in
potentially incompatible ways, for which we'll just break compatibility and
ship new major versions. Over time and as the proxy-wasm ABI spec and
development stabilizes, we expect the need for this fork to fade away.

If you want to use this proxy-wasm fork, make sure to pin your project to a
specific tag or a `-stable` release. If you want to minimise diffs when you
switch over to upstream at a later point, be sure to use package renaming in
your cargo manifest's dependencies section:

> proxy-wasm = { git = "https://github.com/3scale/proxy-wasm-rust-sdk", branch = "v0.1-stable", package = "proxy-wasm-3scale" }

## Differences

So far the main difference is that this fork does not support `set_*_context`
calls, but instead requires the developer to implement `on_create_child_context`
if they intend to create HTTP or Stream contexts.

Additionally, this fork uses FilterStatus (and other similar types) instead of
Action for returning status values back to Envoy. This is so we can avoid some
bugs in filters by using values not exposed by Action. That said, at this
moment such bugs might be already fixed and we could soon return to use Action.

## Contributing

While the plan is to maintain this project as a lightweight fork on top of
upstream, we welcome PRs, but we request contributors to make sure any new
code is also PR'ed to the upstream project unless it touches something that
upstream won't add or is undecided or unresponsive about.

## Examples

+ [Hello World](./examples/hello_world.rs)
+ [HTTP Auth random](./examples/http_auth_random.rs)
+ [HTTP Headers](./examples/http_headers.rs)

## Articles & blog posts from the community

+ [Extending Envoy with WASM and Rust](https://antweiss.com/blog/extending-envoy-with-wasm-and-rust/)
+ [Extending Istio with Rust and WebAssembly](https://blog.red-badger.com/extending-istio-with-rust-and-webassembly)

## Updating dependencies

When updating dependencies, you need to regenerate Bazel `BUILD` files to match updated `Cargo.toml`:
```
cargo install cargo-raze --version 0.7.0
rm -rf bazel/cargo/
cargo generate-lockfile
cargo raze --output=bazel/cargo
mv Cargo.lock bazel/cargo/
```
