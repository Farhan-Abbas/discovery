#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, _itm) = aux11::init();

    loop {
        // wait until it's safe to write to TDR
        while usart1.isr.read().txe().bit_is_clear() {}
        // write the byte to TDR
        usart1.tdr.write(|w| w.tdr().bits(u16::from('a' as u8)));
        
        // Wait until there's data available
        while usart1.isr.read().rxne().bit_is_clear() {}
        // Retrieve the data
        let _byte = usart1.rdr.read().rdr().bits() as u8;

        aux11::bkpt();
    }
}