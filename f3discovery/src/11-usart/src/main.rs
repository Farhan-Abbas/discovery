// Code for section 11.6: Echo Server
// #![deny(unsafe_code)]
// #![no_main]
// #![no_std]

// #[allow(unused_imports)]
// use aux11::{entry, iprint, iprintln};

// #[entry]
// fn main() -> ! {
//     let (usart1, _mono_timer, _itm) = aux11::init();

//     loop {
//         // Wait until there's data available
//         while usart1.isr.read().rxne().bit_is_clear() {}

//         // Retrieve the data
//         let _byte = usart1.rdr.read().rdr().bits() as u8;

//         // Send a single character
//         usart1.tdr.write(|w| w.tdr().bits(u16::from(_byte)) );
//     }
// }

// Code for section 11.7: Reverse a string
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};
use heapless::Vec;

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, _itm) = aux11::init();

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, 32> = Vec::new();
    
    loop {
        buffer.clear();

        let mut string = "";

        // TODO Receive a user request. Each user request ends with ENTER
        // NOTE `buffer.push` returns a `Result`. Handle the error by responding
        // with an error message.

        // TODO Send back the reversed string

        while true {
            // Wait until there's data available
            while usart1.isr.read().rxne().bit_is_clear() {}
            // Retrieve the data
            let _byte = usart1.rdr.read().rdr().bits() as u8;
            if _byte == b'\n' {
                break;
            }
            // push the byte into the buffer
            string.push(_byte);
        }

        // reverse the string

        // return the reversed string
        for byte in string.chars() {
            // wait until it's safe to write to TDR
            while usart1.isr.read().txe().bit_is_clear() {}
            usart1.tdr.write(|w| w.tdr().bits(u8::from(byte)));
        }


    }
}
