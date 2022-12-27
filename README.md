# Joystick Controller Decoder written in Rust

This is a rust port of a c program that I essentially ripped from the
linux joystick api docs to decode the information that a controller
sends to a host machine.

Originally, when making the c implementation, I thought that USB and
Bluetooth controllers would show up differently in linux. I had no way
to test this as I only had a wired controller at the time. 
It seems as though the devices show up the same under `/dev/input/`.

This means that, theoretically, this should work with any? controller.

Tested with XBox Series Bluetooth Controller

Made this in an attempt to learn rust.

[C implementation](https://github.com/ShahriyarShawon/USBControllerDecoder)

[Linux Joystick API](https://www.kernel.org/doc/Documentation/input/joystick-api.txt)

## Test Run
```bash
cargo run -- /dev/input/jsx/
```
Replace x with the number your controller is assigned to.
Most likely it will be zero making the run command 
`cargo run -- /dev/input/js0`

I found this device running 
```bash
ls /dev/input/
``` 
theres a bunch of stuff in there

