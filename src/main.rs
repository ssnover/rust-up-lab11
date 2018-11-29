/*
 * file: main.rs
 * purpose: Entry point of the application. Defines the panic handler, sets up
 *          peripherals for the application state machine, and starts the state
 *          machine.
 */

#![no_std]

extern crate msp430;
extern crate msp430g2553;

use msp430::{asm, interrupt};
use msp430g2553::PORT_1_2;
use core::panic::PanicInfo;

mod watchdog;

fn delay(n: u16) {
    let mut i = 0;
    loop {
        asm::nop();

        i += 1;

        if i == n {
            break;
        }
    }
}

/*
 * Entry point of the application. Initializes peripherals and executes the
 * application state machine.
 */
fn main() {
    // Disable the watchdog
    watchdog::disable();

    interrupt::free(|cs| {
        let port_1_2 = PORT_1_2.borrow(cs);

        // set P0 high and P6 low
        port_1_2
            .p1out
            .modify(|_, w| w.p0().set_bit().p6().clear_bit());

        // Set P0 and P6 as outputs
        port_1_2
            .p1dir
            .modify(|_, w| w.p0().set_bit().p6().set_bit());

        loop {
            delay(30_000);

            // toggle outputs
            port_1_2.p1out.modify(
                |r, w| w.p0().bit(!r.p0().bit()).p6().bit(!r.p6().bit()),
            );
        }
    });
}

/* 
 * On a panic, this function executes to prevent continued undefined behavior.
 */
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
