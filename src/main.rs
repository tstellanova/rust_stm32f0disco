#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
extern crate stm32f0;
use stm32f0::stm32f0x1;  //stm32f051


use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_semihosting::{hprintln};


#[entry]
fn main() -> ! {
  hprintln!("Hello, world!").unwrap();
  
  // let rcc = RCC::borrow();
  // rcc.ahbenr.modify(|_, w| w.iopcenr().enabled());
  
  let f0x1_periphs = stm32f0x1::Peripherals::take().unwrap();
  let gpioc = &f0x1_periphs.GPIOC;
  // let p = stm32f40x::Peripherals::take().unwrap();
  // p.RCC.ahb1enr.write(|w| w.gpioeen().enabled());
  f0x1_periphs.RCC.ahbenr.write(|w| w.iopcen().set_bit());
  
  /*
  #define PORT_LED GPIOC
  #define PIN_LED GPIO8
  gpio_mode_setup(PORT_LED, GPIO_MODE_OUTPUT, GPIO_PUPD_NONE, PIN_LED);
  OTYPER     PushPull: [0, "Output push-pull (reset state)"]
  */

  //    RCC->AHBENR |= RCC_AHBENR_GPIOCEN;
  //                    ahb.enr().modify(|_, w| w.$iopxenr().enabled());

  // rcc.iopcen();

  //configure all as inputs
  gpioc.moder.reset();
  
  //set as outputs
  // gpioc.moder.modify(|_, w| w.moder7().output());
  gpioc.moder.modify(|_, w| w.moder8().output());
  gpioc.moder.modify(|_, w| w.moder9().output());
  // set as push pull
  // gpioc.otyper.modify(|_, w| w.ot7().push_pull());
  gpioc.otyper.modify(|_, w| w.ot8().push_pull());
  gpioc.otyper.modify(|_, w| w.ot9().push_pull());
  //set as no pull up or down
  // gpioc.pupdr.modify(|_, w| w.pupdr7().floating());
  gpioc.pupdr.modify(|_, w| w.pupdr8().floating());
  gpioc.pupdr.modify(|_, w| w.pupdr9().floating());
  
  // gpioc.bsrr.write(|w| unsafe { w.bits(0xFFFFFFFF) } );
  // gpioc.odr.modify(|_, w| w.odr7().high());
  gpioc.odr.modify(|_, w| w.odr8().high());
  gpioc.odr.modify(|_, w| w.odr9().high());
    

  // configure the system timer to wrap around every second
  let m_periphs = cortex_m::Peripherals::take().unwrap();
  let mut syst = m_periphs.SYST;
  syst.set_clock_source(SystClkSource::Core);
  syst.set_reload(8_000_000); // 1s
  syst.enable_counter();

  loop {
    //i2c1.cr2.modify(|r, w| w.stop().bit(!r.stop().bit()));
    // GPIOC->BSRR = BSRR_VAL;
    gpioc.bsrr.write(|w| unsafe { w.bits(0x0300) });
    while !syst.has_wrapped() {}
    // GPIOC->BRR = BSRR_VAL;
    gpioc.brr.write(|w| unsafe { w.bits(0x0300) });
    
    // gpioc.odr.modify(|r, w| w.odr8().bit(!r.odr8().bit()));
    // gpioc.odr.modify(|r, w| w.odr9().bit(!r.odr9().bit()));

      // busy wait until the timer wraps around
    while !syst.has_wrapped() {}
    hprintln!(".").unwrap();
    continue;
  }

}
