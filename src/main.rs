#![no_std]
#![no_main]

use hal::{gpio::Level, prelude::OutputPin};
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;

use defmt_rtt as _; // global logger
use nrf52840_hal as hal;

#[entry]
fn main() -> ! {
    defmt::info!("hello leds");

    if let Some(periph) = hal::pac::Peripherals::take() {
        let gpio_pins = hal::gpio::p0::Parts::new(periph.P0);
        let mut led1 = gpio_pins.p0_13.degrade().into_push_pull_output(Level::High);
        led1.set_low().unwrap();
    }

    // make fn divergent
    loop {}
}
