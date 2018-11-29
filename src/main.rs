#![no_std]

extern crate msp430;
extern crate msp430g2553;

use msp430::{asm, interrupt};
use msp430g2553::PORT_1_2;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

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

// P0 = red LED
// P6 = green LED
fn main() {
    interrupt::free(|cs| {
        // Disable watchdog
        let wdt = msp430g2553::WATCHDOG_TIMER.borrow(&cs);
        wdt.wdtctl.write(|w| {
            unsafe { w.bits(0x5A00) } // password
            .wdthold().set_bit()
        });

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
