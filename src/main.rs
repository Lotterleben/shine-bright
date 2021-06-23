#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _; // global logger
use panic_halt as _;

use diamond::Leds;

#[entry]
fn main() -> ! {
    defmt::info!("hello leds");

    let mut leds = Leds::init().unwrap();

    let mut my_led = leds.top_right;
    my_led.on();

    //✏️ This will lead to a compiler error: the top_right led has already been taken
    //let my_second_led = leds.top_right;

    // this LED stays on forever:
    leds.bottom_right.on(); // make it shine
    drop(leds.bottom_right); // make sure we can't use it anymore after his

    // make fn divergent
    loop {}
}
