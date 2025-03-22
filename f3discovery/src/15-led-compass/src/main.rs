// Take 1 - Solution
// #![no_main]
// #![no_std]

// use compass_lsm303agr::config::initialization::{
//     entry, init, iprintln, switch_hal::OutputSwitch, Direction,
// };
// use stm32f3_discovery::stm32f3xx_hal::prelude::*;

// #[entry]
// fn main() -> ! {
//     let (leds, mut lsm303agr, mut delay, mut itm) = init();
//     let mut stm_leds = leds.into_array();

//     loop {
//         let lsm303agr::UnscaledMeasurement { x, y, .. } = lsm303agr.mag_data().unwrap();
//         // Look at the signs of the X and Y components to determine in which
//         // quadrant the magnetic field is
//         let dir = match (x > 0, y > 0) {
//             // Quadrant I
//             (true, true) => Direction::Southeast,
//             // Quadrant II
//             (false, true) => Direction::Northeast,
//             // Quadrant III
//             (false, false) => Direction::Northwest,
//             // Quadrant IV
//             (true, false) => Direction::Southwest,
//         };

//         stm_leds.iter_mut().for_each(|led| led.off().unwrap());
//         stm_leds[dir as usize].on().unwrap();
        
//         delay.delay_ms(1_000_u16);
//     }
// }

// Take 2 - Solution
#![deny(unsafe_code)]
#![no_main]
#![no_std]

// You'll find this useful ;-)
use core::f32::consts::PI;

#[allow(unused_imports)]
// use aux15::{entry, iprint, iprintln, prelude::*, switch_hal::OutputSwitch, Direction, I16x3};
use compass_lsm303agr::config::initialization::{
    entry, init, iprintln, switch_hal::OutputSwitch, Direction,
};
use m::Float;
use stm32f3_discovery::stm32f3xx_hal::prelude::*;


#[entry]
fn main() -> ! {
    let (leds, mut lsm303agr, mut delay, mut itm) = init();
    // let (leds, mut lsm303dlhc, mut delay, _itm) = aux15::init();
    let mut leds = leds.into_array();

    loop {
        let lsm303agr::UnscaledMeasurement { x, y, .. } = lsm303agr.mag_data().unwrap();
        // let I16x3 { x, y, .. } = lsm303dlhc.mag().unwrap();

        let theta = (y as f32).atan2(x as f32); // in radians

        let dir = if theta < -7. * PI / 8. {
            Direction::North
        } else if theta < -5. * PI / 8. {
            Direction::Northwest
        } else if theta < -3. * PI / 8. {
            Direction::West
        } else if theta < -PI / 8. {
            Direction::Southwest
        } else if theta < PI / 8. {
            Direction::South
        } else if theta < 3. * PI / 8. {
            Direction::Southeast
        } else if theta < 5. * PI / 8. {
            Direction::East
        } else if theta < 7. * PI / 8. {
            Direction::Northeast
        } else {
            Direction::North
        };

        leds.iter_mut().for_each(|led| led.off().unwrap());
        leds[dir as usize].on().unwrap();

        delay.delay_ms(100_u8);
    }
}