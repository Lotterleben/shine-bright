# Shine Bright (Like a Diamond)

## Dependencies

- This example is designed to be ran on an [nRF528540 Development Kit]
- [Rust](https://www.rust-lang.org/tools/install)
- [probe-run](https://github.com/knurling-rs/probe-run): 
```console
$ cargo install probe-run
```

## Usage

1. Connect your [nRF528540 Development Kit] to your computer using the micro-USB port on the *short side* of your DK

2. Flash and run

``` console
$ cd shine-bright
$ cargo run
```

3. Observe results

In the console, you should see:

```console
   Compiling diamond v0.1.0 (/Users/lottesteenbrink/talks/muc++/shine-bright/diamond)
   Compiling shine-bright v0.1.0 (/Users/lottesteenbrink/talks/muc++/shine-bright)
    Finished dev [unoptimized + debuginfo] target(s) in 1.23s
     Running `probe-run --chip nrf52840 target/thumbv7em-none-eabihf/debug/shine-bright`
(HOST) INFO  flashing program (12.33 KiB)
(HOST) INFO  success!
────────────────────────────────────────────────────────────────────────────────
 INFO  hello leds
└─ shine_bright::__cortex_m_rt_main @ src/main.rs:12
```

On the [nRF528540 Development Kit], you should see

TODO add gif of blinking LEDs here



## Background Information

This repository was initially created using [cortex-m-quickstart](https://github.com/rust-embedded/cortex-m-quickstart)


[nRF528540 Development Kit]: https://www.nordicsemi.com/Software-and-Tools/Development-Kits/nRF52840-DK