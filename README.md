# beep

This repo contains both a library and an executable implementation of said library.

It beeps.

I found the `wild_eep.wav` from MacOS of days long gone. The library packages the wav into the binary, and plays it
using the `rodio` crate. The binary does nothing more than call `beep::beep()`.

That's the whole of it.

The current method is blocking (>=850ms).

## TODO

- add a second library that implements a `tokio` async version of `beep::beep()`
