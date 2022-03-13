## Sample project

Sample project for a ESP32-C3 embedded program, shows how to use:
  - A LCD display based on SSD1036, connected to GPIO4 and GPIO5
  - Connecting to a client wifi

## Getting started

Project is based on the [esp-rs/esp-idf-template](https://github.com/esp-rs/esp-idf-template) 
project. Read its documentation for getting started. I was generally started as

```console
$ rustup install nightly
$ rustup component add rust-src --toolchain nightly

$ cargo install cargo-generate
$ cargo install ldproxy
$ cargo install espflash
$ cargo install espmonitor

$ cargo generate --vcs none --git https://github.com/esp-rs/esp-idf-template cargo
```
