#![no_std]

use nrf52840_hal::{
    gpio::{p0::Parts, Level, Output, Pin, PushPull},
    prelude::OutputPin,
};

// wishlist
// - ZeroSizedLeds
// - Blinkable trait impl'ed by both led types
// - talk about `#[must_use]` types instead of just "hey this is a (void)"?
// - basic defmt-tests

#[derive(Debug)]
pub enum LedError {
    PeripheralsAlreadyInUse,
}

// Grants access to the four on-board LEDs of the nrf52840 Development Kit
pub struct Leds {
    pub top_left: Led,
    pub top_right: Led,
    pub bottom_left: Led,
    pub bottom_right: Led,
}

pub struct Led {
    pin: Pin<Output<PushPull>>,
}

impl Led {
    pub fn on(&mut self) {
        let _ = self.pin.set_low();
        //✏️^ this is the equivalent of casting to void to silence the compiler
    }

    pub fn off(&mut self) {
        let _ = self.pin.set_high();
    }
}

impl Leds {
    pub fn init() -> Result<Leds, LedError> {
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
