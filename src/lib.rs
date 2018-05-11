//! Report panic messages to the host stderr using semihosting
//!
//! This crate contains an implementation of `panic_fmt` that logs panic messages to the host stderr
//! using [`cortex-m-semihosting`]. Before logging the message the panic handler disables (masks)
//! the device specific interrupts. After logging the message the panic handler trigger a breakpoint
//! and then goes into an infinite loop.
//!
//! Currently, this crate only supports the ARM Cortex-M architecture.
//!
//! [`cortex-m-semihosting`]: https://crates.io/crates/cortex-m-semihosting
//!
//! # Requirements
//!
//! To build this crate on the stable or beta channels `arm-none-eabi-gcc` needs to be installed and
//! available in `$PATH`.
//!
//! # Usage
//!
//! ``` ignore
//! #![no_std]
//!
//! extern crate panic_semihosting;
//!
//! fn main() {
//!     panic!("FOO")
//! }
//! ```
//!
//! ``` text
//! (gdb) monitor arm semihosting enable
//! (gdb) continue
//! Program received signal SIGTRAP, Trace/breakpoint trap.
//! rust_begin_unwind (args=..., file=..., line=8, col=5)
//!     at $CRATE/src/lib.rs:69
//! 69          asm::bkpt();
//! ```
//!
//! ``` text
//! $ openocd -f (..)
//! (..)
//! panicked at 'FOO', src/main.rs:6:5
//! ```
//!
//! # Optional features
//!
//! ## `inline-asm`
//!
//! When this feature is enabled semihosting is implemented using inline assembly (`asm!`) and
//! compiling this crate requires nightly.
//!
//! When this feature is disabled semihosting is implemented using FFI calls into an external
//! assembly file and compiling this crate works on stable and beta.
//!
//! Apart from the toolchain requirement, enabling `inline-asm` removes the requirement of having
//! `arm-none-eabi-gcc` installed on the host.

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(lang_items)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_semihosting as sh;

use core::fmt::{self, Write};
use cortex_m::{asm, interrupt};

use sh::hio;

#[lang = "panic_fmt"]
unsafe extern "C" fn panic_fmt(
    args: core::fmt::Arguments,
    file: &'static str,
    line: u32,
    col: u32,
) -> ! {
    interrupt::disable();

    if let Ok(mut hstdout) = hio::hstdout() {
        (|| -> Result<(), fmt::Error> {
            hstdout.write_str("panicked at '")?;
            hstdout.write_fmt(args)?;
            hstdout.write_str("', ")?;
            hstdout.write_str(file)?;
            writeln!(hstdout, ":{}:{}", line, col)
        })().ok();
    }

    // OK to fire a breakpoint here because we know the microcontroller is connected to a debugger
    asm::bkpt();

    loop {}
}
