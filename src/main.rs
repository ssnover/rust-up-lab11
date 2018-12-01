/*
 * file: main.rs
 * purpose: Entry point of the application. Defines the panic handler, sets up
 *          peripherals for the application state machine, and starts the state
 *          machine.
 */

#![no_std]

#![feature(abi_msp430_interrupt)]
extern crate msp430;
#[macro_use(interrupt)]
extern crate msp430g2553;

use msp430g2553::PORT_1_2;
use core::panic::PanicInfo;

mod watchdog;
mod system_tick;

fn check_for_system_tick() -> bool {
    unsafe { system_tick::FLAG }
}

/*
 * Entry point of the application. Initializes peripherals and executes the
 * application state machine.
 */
fn main() {
    // Disable the watchdog
    watchdog::disable();
    unsafe {
        msp430::giinterrupt::enable();
    }

    // Set up the timer for the system tick to run at 100 Hz
    system_tick::init(100);
    system_tick::enable();

    let mut counter: u8 = 0;

    loop {
        if check_for_system_tick() {
            unsafe { system_tick::FLAG = false; } 
            // execute state machine
            if counter == 100 {
                counter = 0;
                toggle_leds();
            }
            counter += 1;
        }
    }
/*
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
    });

    loop {
        delay(30_000);
        // toggle outputs
        toggle_leds();
    }
    */
}

fn toggle_leds() {
    msp430::interrupt::free(|cs| {
        let gpio_port = PORT_1_2.borrow(&cs);
        gpio_port.p1out.modify(
            |r, w| w.p0().bit(!r.p0().bit()).p6().bit(!r.p6().bit()),
        );
    });
}

/* 
 * On a panic, this function executes to prevent continued undefined behavior.
 */
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
