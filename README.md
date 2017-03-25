# Chat

A simple relay between multiple clients, built as a demo project for the [`comms` library](https://github.com/46bit/comms).

Multiple clients connect over TCP and type to each other - [demo video](https://www.youtube.com/watch?v=I2j2aLBo2Es).

## Running

You'll need Rust and to clone this repo. Then run `cargo run` and you should see the server build and start up.

## Talking

* To live-type, run a few terminal windows of `stty -icanon && nc localhost 8080` and type into them.
* To talk a line at a time, run a few terminal windows of `nc localhost 8080` and type into them followed by a newline.

You can mix both - the buffering is on the client side.
