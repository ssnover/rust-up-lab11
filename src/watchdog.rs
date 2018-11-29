/*
 * file: watchdog.rs
 * purpose: Interface for the MSP430's watchdog.
 */

extern crate msp430;
extern crate msp430g2553;

/*
 * Disables the hardware watchdog timer to prevent spurious resets of the
 * microcontroller from not petting the watchdog.
 */
pub fn disable() {
    msp430::interrupt::free(|cs| {
        let wdt = msp430g2553::WATCHDOG_TIMER.borrow(&cs);
        wdt.wdtctl.modify(|_, w| {
            unsafe { w.bits(0x5A00) }
            .wdthold().set_bit()
        });
    });
}