/*
 * file: system_tick.rs
 * purpose: Interface for a system tick using a hardware timer.
 */

extern crate msp430;
extern crate msp430g2553;

//const SYSTEM_MAIN_FREQUENCY_HZ: u32 = 1000000;
pub static mut FLAG: bool = false;

 /*
  * Initializes the system tick with the input frequency.
  */
pub fn init(_frequency: u16) {

    /*
     * So using multiplication and division causes build failures during
     * linking. Division here required __mspabi_divul and multiplication
     * required __mspabi_mpyl.
     */
    //let divider: u16 = ((SYSTEM_MAIN_FREQUENCY_HZ / frequency as u32) & 0x0000FFFF) as u16;
    let divider: u16 = 0x268 << 1;

    msp430::interrupt::free(|cs| {
        let timer1 = msp430g2553::TIMER1_A3.borrow(&cs);
        timer1.ta1ctl.modify(|_, w| {
            w.tassel().tassel_2()
             .id().id_2()
             .mc().mc_1()
        });
        timer1.ta1cctl0.modify(|_, w| {
            w.cm().cm_0()
             .cap().clear_bit()
             .ccifg().clear_bit()
        });
        timer1.ta1ccr0.write(|w| unsafe { w.bits(divider)});
    });
}

/*
 * Enables the system tick.
 */
pub fn enable() {
    msp430::interrupt::free(|cs| {
        let timer = msp430g2553::TIMER1_A3.borrow(&cs);
        timer.ta1cctl0.modify(|_, w| w.ccie().set_bit())
    });
}

/*
 * Disables the system tick.
 */
fn _disable() {
    msp430::interrupt::free(|cs| {
        let timer = msp430g2553::TIMER1_A3.borrow(&cs);
        timer.ta1cctl0.modify(|_, w| w.ccie().clear_bit())
    });
}

/*
 * Interrupt handler for the system tick interrupt.
 */
interrupt!(TIMER1_A1, systick_handler);
fn systick_handler() {
    msp430::interrupt::free(|cs| {
        let timer = msp430g2553::TIMER1_A3.borrow(cs);
        timer.ta1cctl1.modify(|_, w| w.ccifg().clear_bit());
    });

    unsafe { FLAG = true; }
}
