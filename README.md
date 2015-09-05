# Rack endpoint in Rust

This project consists of a Ruby module called `RustRack`, implemented
entirely in Rust, which has a `#call` method that conforms to the Rack
protocol. A Rackup file is provided that mounts the module to serve
requests.

To compile:

    $ make

This creates a shim Ruby native extension (lib/rust_rack.bundle) that
links against the compiled Rust library. The included config.ru will
then work when you run `rackup`.
