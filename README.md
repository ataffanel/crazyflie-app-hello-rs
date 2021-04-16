# Hello world Rust App for Crazyflie 2.X

Experimental example a Crazyflie app in Rust.

This prints a hello message on the Crazyflie console and then sleeps forever.
Integration with the Crazyflie and FreeRTOS API is non-existent at the moment, this is only a starting point.

## Build

Since this app includes the Crazyflie firmware as a subrepos it needs to be cloned with `git clone --recursive`.
It is also possible to run `git submodule update --init --recursive` in an already cloned folder to make sure everything is cloned.

The prerequisite to build the rust app is to have a recent rust compiler (tested with 1.51) and the target for the Crazyflie CPU.
To install the target with [rustup](https://rustup.rs):
```
rustup target add thumbv7em-none-eabihf
```

Then, the classic `make && make cload` will build and flash the firmware together with the app:
```
make
make cload
```

This currently needs the Rust nightly compiler to function, read more [here](https://doc.rust-lang.org/1.2.0/book/nightly-rust.html).
## Architecture

The Crazyflie build system allows to add objects to be linked with the Crazyflie.

The Rust project is setup to generate a C static library.
The Makefile calls `cargo build` to build the lib and adds it to the list of objects to be linked in the Crazyflie firmware.

The Crazyflie firmware will call `appMain()` at startup.
This function is declared in rust as `pub extern "C"` to be callable from C.

The FreeRTOS delay function and Crazyflie console putchar function are manually declared `extern "C"` which allows Rust to call them.
Future improvement includes using a FreeRTOS binding and possibly making a Crazflie binding so that Rust app can do useful things.