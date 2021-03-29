# The Oasis

The Oasis is written in Rust and operates on a multithreaded TCP server

### Running the server
Invoke in the terminal and then load 127.0.0.1:7878 in a web browser.

    cargo run 

This code will listen at the address 127.0.0.1:7878 for incoming TCP streams.

Keep server running

    cargo watch -x run