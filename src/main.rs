#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _; // global logger
use nrf52840_hal::{
    gpio::{p0::Parts, Level, Output, Pin, PushPull},
    prelude::OutputPin,
};
use panic_halt as _;

#[derive(Debug)]
pub enum LedError {
    PeripheralsAlreadyInUse,
}

// wishlist
// - ZeroSizedLeds
// - immutable leds that are fixed in their first state
// - Blinkable trait impl'ed by both led types

// Grants access to the four on-board LEDs of the nrf52840 Development Kit
pub struct Leds {
    top_left: Led,
    top_right: Led,
    bottom_left: Led,
    bottom_right: Led,
}

pub struct Led {
    pin: Pin<Output<PushPull>>,
}

impl Led {
    fn on(&mut self) {
        let _ = self.pin.set_low();
        //✏️^ this is the equivalent of casting to void to silence the compiler
    }
}

impl Leds {
    fn init() -> Result<Leds, LedError> {
        if let Some(periph) = nrf52840_hal::pac::Peripherals::take() {
            let pins = Parts::new(periph.P0);

            // Note: `Level::High` turns the LEDs *off*
            let pin_1 = pins.p0_13.degrade().into_push_pull_output(Level::High);
            let pin_2 = pins.p0_14.degrade().into_push_pull_output(Level::High);
            let pin_3 = pins.p0_15.degrade().into_push_pull_output(Level::High);
            let pin_4 = pins.p0_16.degrade().into_push_pull_output(Level::High);

            Ok(Self {
                top_left: Led { pin: pin_1 },
                top_right: Led { pin: pin_2 },
                bottom_left: Led { pin: pin_3 },
                bottom_right: Led { pin: pin_4 },
            })
        } else {
            Err(LedError::PeripheralsAlreadyInUse)
        }
    }
}

#[entry]
fn main() -> ! {
    defmt::info!("hello leds");

    let leds = Leds::init().unwrap();

    let mut my_led = leds.top_right;

    my_led.on();

    //✏️ This will lead to a compiler error: the top_right led has already been taken
    //let my_second_led = leds.top_right;

    // make fn divergent
    loop {}
}
