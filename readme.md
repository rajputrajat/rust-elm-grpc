# Experimenting interop between Rust (wasm) and Elm

## want to try both way communication

* from Elm app click a button which will send a message through JS-port to rust-wasm, \
which should deliver that message over grpc to other side of the solution

* from other side of the solution, send message to elm app and do some action on webpage

* will do this on network having different OS (and then vice-versa).
