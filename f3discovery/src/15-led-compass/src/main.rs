#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux15::{entry, iprint, iprintln, prelude::*, UnscaledMeasurement};
use aux15::Direction;

#[entry]
fn main() -> ! {
    let (mut leds, mut lsm303agr, mut delay, mut itm) = aux15::init();

    loop {
        let status = lsm303agr.mag_status().unwrap();
        if status.xyz_new_data {
            let data = lsm303agr.mag_data().unwrap();
            iprintln!(&mut itm.stim[0], "{:?}", data);
        }

        delay.delay_ms(1_000_u16);
    }
}