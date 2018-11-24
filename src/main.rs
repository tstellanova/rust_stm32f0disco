#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // enable brkpt on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
extern crate stm32f0;
use stm32f0::stm32f0x1;  //stm32f051 for stm32f0discovery board


use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_semihosting::{hprintln};


/// Configure GPIOC, including LED3 and LED4 outputs for stm32f0discovery board
fn gpioc_setup(gpioc: &stm32f0x1::GPIOC) {
  
  //reset all as inputs
  gpioc.moder.reset();
  
  //set as outputs
  gpioc.moder.modify(|_, w| {
    w.moder8().output();
    w.moder9().output()
  });
    
  // set as push pull
  gpioc.otyper.modify(|_, w| {
    w.ot8().push_pull();
    w.ot9().push_pull()
  });
  
  //set as no pull up or down (floating)
  gpioc.pupdr.modify(|_, w| {
    w.pupdr8().floating();
    w.pupdr9().floating()
  });
  
  //initialize output values
  gpioc.odr.modify(|_, w| {
    w.odr8().low();
    w.odr9().high()
  });
  
}

#[entry]
fn main() -> ! {
  hprintln!("begin").unwrap();
  
  let f0x1_periphs = stm32f0x1::Peripherals::take().unwrap();
  let gpioc = &f0x1_periphs.GPIOC;

  // enable the peripheral clock for GPIOC
  f0x1_periphs.RCC.ahbenr.write(|w| w.iopcen().set_bit());
  
  gpioc_setup(gpioc);
  
  // configure the system timer to wrap around every second
  let m_periphs = cortex_m::Peripherals::take().unwrap();
  let mut syst = m_periphs.SYST;
  syst.set_clock_source(SystClkSource::Core);
  syst.set_reload(8_000_000); // 1s, assuming XX mhz clock
  syst.enable_counter();

  loop {
    // flip LED state on each loop
    gpioc.odr.modify(|r, w| {
        w.odr8().bit(!r.odr8().bit());
        w.odr9().bit(!r.odr9().bit())
    });

    // busy wait until the timer wraps around
    while !syst.has_wrapped() {}
    hprintln!(".").unwrap();
  }

}
