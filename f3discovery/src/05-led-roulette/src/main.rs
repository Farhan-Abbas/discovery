#![deny(unsafe_code)]
#![no_main]
#![no_std]

use volatile::Volatile;
use aux5::{Delay, DelayMs, LedArray, OutputSwitch, entry};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let mut period = 50_u16;
    // let v_half_period = Volatile::new(&mut half_period);
    let mut index = 0;
    leds[index].on().ok();
    delay.delay_ms(period);

    loop {
        delay.delay_ms(period);
        index += 1;
        if (index == 8) {
            index = 0;
        }
        leds[index].on().ok();
        delay.delay_ms(period);
        if index == 0 {
            leds[7].off().ok();
        } else {
            leds[index-1].off().ok();
        }
    }
}

