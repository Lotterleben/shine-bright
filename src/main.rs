#![no_std]
#![no_main]

use core::fmt::Debug;

use cortex_m_rt::entry;
use nrf52840_hal::{
    gpio::{p0::Parts, Level, Output, Pin, PushPull},
    prelude::OutputPin,
};

use defmt_rtt as _; // global logger
use panic_halt as _;

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
        self.pin.set_low().unwrap();
        //                 ^^^^^^^^ panic on error
    }

    pub fn off(&mut self) {
        self.pin.set_high().unwrap();
    }
}

impl Leds {
    pub fn init() -> Result<Leds, LedError> {
        if let Some(periph) = nrf52840_hal::pac::Peripherals::take() {
            let pins = Parts::new(periph.P0);

            let pin_1 = pins.p0_13.degrade().into_push_pull_output(Level::High);
            //                          ✏️  ^^^^^^^^^  ^^^^^^    ^^^^^^^^^^^^^^^^^^^^  ^^^^^^
            // get ownership of P0_13 representation     |       configure pin as       High = LED off
            // (nobody else can have it now!)            |       push-pull output
            //                                           |
            //                              degrade Type from "this specific P0_13"
            //                              to "some Pin" for easier handling

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
    // logging statement
    defmt::info!("hello leds");

    let mut leds = Leds::init().unwrap();

    let mut my_led = leds.top_right;
    my_led.on();

    //✏️ This will lead to a compiler error: `my_led` already owns the `top_right` led
    //let my_second_led = leds.top_right;

    // this LED stays on forever:
    leds.bottom_right.on(); // make it shine
    drop(leds.bottom_right); // make sure we can't use it anymore after his

    // ✏️ This will lead to a compiler error: nobody can take ownership of `bottom_right`,
    // because it has been thrown away forever
    leds.bottom_right.off();

    // make fn divergent (i.e. it never stops)
    loop {}
}
