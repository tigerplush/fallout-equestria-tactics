# fallout-equestria-tactics

This is a turn-based strategic multiplayer game.

## Architecture

foe tactics follows a basic server-client architecture. To start the server, use `cargo run --bin server`, to start a client, run `cargo run --bin client`, the client will automatically connect to the server running on localhost:5000. To choose another port for the server, run it with `cargo run --bin server -- ip:port`