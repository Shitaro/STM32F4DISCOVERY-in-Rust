#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use stm32f4::stm32f407;

#[entry]
fn main() -> ! {

    // Obtain the peripheral object from stm32f407 module
    let perip = stm32f407::Peripherals::take().unwrap();

    //  Enable the clock to GPIOD
    perip.RCC.ahb1enr.modify(|_,w| w.gpioden().set_bit());

    // Set pins 12-15 to be general purpose output
    let gpiod = &perip.GPIOD;
    gpiod.moder.modify(|_,w| w.moder12().output());
    gpiod.moder.modify(|_,w| w.moder13().output());
    gpiod.moder.modify(|_,w| w.moder14().output());
    gpiod.moder.modify(|_,w| w.moder15().output());

    loop {
        // Set pins 12-15 to be high level
        gpiod.bsrr.write(|w| w.bs12().set_bit());
        gpiod.bsrr.write(|w| w.bs13().set_bit());
        gpiod.bsrr.write(|w| w.bs14().set_bit());
        gpiod.bsrr.write(|w| w.bs15().set_bit());

        // Wait
        delay(8_000_000);

        // Set pins 12-15 to be low level
        gpiod.bsrr.write(|w| w.br12().reset());
        gpiod.bsrr.write(|w| w.br13().reset());
        gpiod.bsrr.write(|w| w.br14().reset());
        gpiod.bsrr.write(|w| w.br15().reset());

        // Wait
        delay(8_000_000);
    }
}