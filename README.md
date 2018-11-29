# Rusted EEEE 420 Lab 11
This project seeks to implement the functional requirements of RIT EEEE 420's final lab on the MSP430 in Rust. The lab culminates a series of labs teaching students to use the peripherals on the MSP430G2 Launchpad to teach embedded systems development in MSP430 assembly and C.

## Requirements
Using the MSP430 Launchpad and the Touchpad daughterboard, the lab requires students to implement the following sequence:

* Prompt the user to press the center touch surface.
* Prompt the user to press either the top or bottom touch surface.
    * The top surface selects a sampling rate of 100 Hz.
    * The bottom surface selects a sampling rate of 200 Hz.
* Prompt the user to press either the left or right touch surface.
    * The left surface selects a linear scale processing for display.
    * The right surface selects a logarithmic scale processing for display.
* Sample an ADC pin 20 times at the selected frequency.
* Apply the selected scale to the sampled data.
* Display the resulting values with the 8 LEDs on the touchpad; each sample is displayed for 500 ms.
* As each sample is displayed on the touchpad, print the number of LEDs being enabled over UART to a serial terminal.
* Continually display and report the values in a loop until interrupted by the user by pressing the center button.
* All timing is to be controlled with hardware, not with a software "busy loop."