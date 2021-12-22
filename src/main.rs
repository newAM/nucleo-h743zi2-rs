#![no_std]
#![no_main]

use defmt_rtt as _; // global logger
use panic_probe as _;
use stm32h7xx_hal as _; // panic handler

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    loop {
        cortex_m::asm::bkpt();
    }
}
