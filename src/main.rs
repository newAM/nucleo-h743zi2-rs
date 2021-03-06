//! Rust boilerplate for the Nucleo-H743ZI2 development board.
//!
//! This board uses the [STM32H7432] MCU.
//!
//! # Linux Probe Setup
//!
//! These are the [udev rules] I use for the on-board STLINK-V3 probe.
//!
//! Create this file:
//!
//! ```text
//! # /etc/udev/rules.d/99-stm.rules
//! SUBSYSTEM=="usb", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374e", MODE="0666"
//! ```
//!
//! Then reload the rules:
//!
//! ```text
//! sudo udevadm control --reload-rules
//! sudo udevadm trigger
//! ```
//!
//! # Running
//!
//! I assume you will use the included probe on the nucleo board (the USB port
//! opposite the Ethernet jack).
//!
//! Use [cargo-embed] to flash the MCU:
//!
//! ```text
//! cargo embed
//! ```
//!
//! That will flash the MCU, and when complete it will bring up an RTT terminal
//! with the logging output from the MCU.
//!
//! [cargo-embed]: https://crates.io/crates/cargo-embed
//! [STM32H7432]: https://www.st.com/resource/en/datasheet/stm32h743vi.pdf
//! [udev rules]: https://wiki.debian.org/udev

#![no_std]
#![no_main]

use core::fmt::Write;
use core::sync::atomic::{compiler_fence, Ordering::SeqCst};

use panic_rtt_target as _;
use rtt_target::rprintln;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut channels = rtt_target::rtt_init! {
        up: {
            0: {
                size: 4096
                mode: BlockIfFull
                name: "Terminal"
            }
        }
    };

    writeln!(&mut channels.up.0, "Hello, world!").ok();

    rprintln!("Hello world!");

    loop {
        compiler_fence(SeqCst);
        // cortex_m::asm::bkpt();
    }
}
