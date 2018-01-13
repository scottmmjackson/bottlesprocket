# Bottlesprocket

## About

bottlesprocket is a port of [bottlerocket](https://github.com/linuxha/bottlerocket) using largely the same approach
but written in Rust.

So far, bottlesprocket has been tested on Ubuntu 16 with a CM17A in ttyS0.

### Huh? What's a CM17A whatsit?

A company called X-10, a long time ago, made these little wall-warts you could plug in to appliances and control
by sending control signals through your AC mains. So you were able to turn devices on or off or dim them by sending
a signal through another plug in your house.

The wall-warts for appliances are called "modules" and the one that sends the signal is called a "transceiver".

The CM17A is a tiny brick that sits in your RS-232 (old school) serial port, and sends a command sequence wirelessly
to the transceiver, which then sends the command through your power lines.

## Installing

### Requirements

- Rust/Cargo: https://www.rustup.rs/
- OS compatible with Rust's libc crate and `TIOCMBIS`/`TIOCMBIC` ioctls

### Instructions

```
cargo install --git https://github.com/scottmmjackson/bottlesprocket
```

## Usage

GOTCHA: Dimming requires some domain knowledge

```
# Turn all my stuff off
bottlesprocket --house A --command ALL_OFF

# Turn my fan on
bottlesprocket --house A --device 3 --command ON

# Dim my bedroom light 30%
# Each DIM command reduces by 5%
bottlesprocket --house A --device 4 --command ON
bottlesprocket --house A --command DIM
bottlesprocket --house A --command DIM
bottlesprocket --house A --command DIM
bottlesprocket --house A --command DIM
bottlesprocket --house A --command DIM

# Brighten my porch light by 20%
# Same idea
bottlesprocket --house A --device 12 --command ON
bottlesprocket --house A --command BRIGHT
bottlesprocket --house A --command BRIGHT
bottlesprocket --house A --command BRIGHT
bottlesprocket --house A --command BRIGHT
```

## Roadmap

- [ ] Make a single convenience command for dimming
- [ ] Make a simple JSON API tool that consumes the same library
