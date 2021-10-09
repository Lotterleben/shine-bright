#![no_std]
#![no_main]

use core::fmt::Debug;

use cortex_m_rt::entry;
use nrf52840_hal::{
    gpio::{p0::Parts, Level, Output, Pin, PushPull},
    pac::TIMER0,
    prelude::OutputPin,
    timer::OneShot,
    Timer,
};

use defmt_rtt as _; // global logger
use panic_halt as _; // defmt-compatible panic handler

const WAIT_CYCLES: u32 = 400_000;

#[derive(Debug)]
enum BoardError {
    PeripheralsAlreadyInUse,
}

/// Represents one single LED
struct Led {
    /// the pin used to control this LED
    pin: Pin<Output<PushPull>>,
}

/// Represents the nrf52840 Development Kit (constrained to the parts we're actually using)
#[allow(dead_code)] // silence warnings about unused leds
struct Board {
    top_left_led: Led,
    top_right_led: Led,
    bottom_left_led: Led,
    bottom_right_led: Led,

    // needed for convenient blinking :)
    timer: Timer<TIMER0, OneShot>,
}

// implements behavior of `Led` instances
impl Led {
    fn on(&mut self) {
        self.pin.set_low().unwrap();
        //                 ^^^^^^^^ note: this panics on error
    }

    fn off(&mut self) {
        self.pin.set_high().unwrap();
    }
}

// implements behavior of `Board` instances
impl Board {
    fn init() -> Result<Board, BoardError> {
        if let Some(periph) = nrf52840_hal::pac::Peripherals::take() {
            let pins = Parts::new(periph.P0);

            let top_left_led_pin = pins.p0_13.degrade().into_push_pull_output(Level::High);
            //                          ✏️             ^^^^^^^^^  ^^^^^^    ^^^^^^^^^^^^^^^^^^^^  ^^^^^^
            //           get ownership of P0_13 representation     |       configure pin as       High = LED off
            //           (nobody else can have it now!)            |       push-pull output
            //                                                     |
            //                                        degrade Type from "this specific P0_13"
            //                                        to "some Pin" for easier handling

            let top_right_led_pin = pins.p0_14.degrade().into_push_pull_output(Level::High);
            let bottom_left_led_pin = pins.p0_15.degrade().into_push_pull_output(Level::High);
            let bottom_right_led_pin = pins.p0_16.degrade().into_push_pull_output(Level::High);

            let timer = nrf52840_hal::Timer::new(periph.TIMER0);

            Ok(Board {
                top_left_led: Led {
                    pin: top_left_led_pin,
                },
                top_right_led: Led {
                    pin: top_right_led_pin,
                },
                bottom_left_led: Led {
                    pin: bottom_left_led_pin,
                },
                bottom_right_led: Led {
                    pin: bottom_right_led_pin,
                },
                timer,
            })
        //  ^^ note: final statements without a `;` are return statements
        //     (we're returning our new `Board` here)
        } else {
            Err(BoardError::PeripheralsAlreadyInUse)
        }
    }
}

#[entry]
fn main() -> ! {
    // logging statement
    defmt::info!("hello leds");

    let mut board = Board::init().unwrap();

    let mut my_led = board.top_right_led;
    my_led.on();

    //✏️ This will lead to a compiler error: `my_led` already owns the `top_right` led
    //let my_second_led = board.top_right;

    // this LED stays on forever:
    board.bottom_right_led.on(); // make it shine
    drop(board.bottom_right_led); // make sure we can't use it anymore after his

    // ✏️ This will lead to a compiler error: nobody can take ownership of `bottom_right`,
    // because it has been thrown away forever
    //board.bottom_right.off();

    // the actual blinking!
    loop {
        board.timer.delay(WAIT_CYCLES);
        my_led.off();
        defmt::info!("⚪️");

        board.timer.delay(WAIT_CYCLES);
        my_led.on();
        defmt::info!("🚨");
    }
}
