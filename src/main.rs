//! magic 8-ball prototype
//! 
//! TO DO: 
//! - get a random answer at a press of a button, display it for some time and then clear screen
//! - get the answer to scroll across the screen
//! 
//! NOTE:
//! - to access the RNG globally I need <rand::rngs::small::SmallRng> as the type
//! 

#![no_std]
#![no_main]


extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_halt;
extern crate stm32f0xx_hal as hal;

use cortex_m_rt::entry;
use cortex_m::interrupt::{free, Mutex};
use core::cell::{Cell, RefCell};

use ssd1306::{prelude::*, Builder as SSD1306Builder};

use crate::hal::{
    prelude::*,
    stm32::
        {self,
        interrupt, 
        Interrupt,
        TIM3,},
    delay::Delay,
    i2c::I2c,
    adc,
    gpio::{gpioa::PA0, Input, PullUp},
    time::Hertz,
    timers::*,
};

use core::fmt::{self, Write};

use rand::prelude::*;

// access the answers from a separate module
mod magicball;
use crate::magicball::answers::answers;

// globally accessible values and peripherals:

// container for the index value
static INDEX: Mutex<Cell<u8>> = Mutex::new(Cell::new(0_u8));

// two consecutive states of the button
static STATE1: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
static STATE2: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

// timer, button, display, RNG

static GTIMER: Mutex<RefCell<Option<Timer<TIM3>>>> = Mutex::new(RefCell::new(None));
static GBUTTON: Mutex<RefCell<Option<PA0<Input<PullUp>>>>> = Mutex::new(RefCell::new(None));
static GDISPLAY: Mutex<RefCell<Option<ssd1306::mode::terminal::TerminalMode<ssd1306::interface::i2c::I2cInterface<hal::i2c::I2c<hal::stm32::I2C1, 
hal::gpio::gpioa::PA9<hal::gpio::Alternate<hal::gpio::AF4>>, hal::gpio::gpioa::PA10<hal::gpio::Alternate<hal::gpio::AF4>>>>>>>> = 
Mutex::new(RefCell::new(None));

//static GRNG: Mutex<RefCell<Option<rand::rngs::small::SmallRng>>> = Mutex::new(RefCell::new(None));

const BOOT_DELAY_MS: u16 = 200; // delay for a correct initialization of the I2C bus/OLED display

#[entry]

fn main() -> ! {
    
    let mut p = stm32::Peripherals::take().unwrap();
    let mut cp = cortex_m::peripheral::Peripherals::take().unwrap();

    cortex_m::interrupt::free(move |cs| {

        //configure the clocks
        let mut rcc = p.RCC.configure().sysclk(8.mhz()).freeze(&mut p.FLASH);

        // configure the delay provider
        let mut delay = Delay::new(cp.SYST, &rcc);

        delay.delay_ms(BOOT_DELAY_MS);

        // configure the ADC
        let mut adc = adc::Adc::new(p.ADC, &mut rcc);

        // configure the I2C bus
        let gpioa = p.GPIOA.split(&mut rcc);
        let scl = gpioa.pa9.into_alternate_af4(cs);
        let sda = gpioa.pa10.into_alternate_af4(cs);
        let i2c = I2c::i2c1(p.I2C1, (scl, sda), 400.khz(), &mut rcc);

        // set up a timer expiring after 50 ms
        let mut btntimer = Timer::tim3(p.TIM3, Hertz(20), &mut(rcc));
        btntimer.listen(Event::TimeOut);

        //move the timer into the global storage
        *GTIMER.borrow(cs).borrow_mut() = Some(btntimer);

        // configure the button and move it to the global storage
        let button = gpioa.pa0.into_pull_up_input(cs);

        *GBUTTON.borrow(cs).borrow_mut() = Some(button);

        //configure the display and move it to the global storage
        let mut disp: TerminalMode<_> = SSD1306Builder::new().size(DisplaySize::Display128x64).connect_i2c(i2c).into();

        disp.init().unwrap();
        disp.clear().unwrap();

        *GDISPLAY.borrow(cs).borrow_mut() = Some(disp);

        // get the internal temperature and voltage values to create a randomized initial seed for the Random Numbers Generator

        let temp = adc::VTemp::read(&mut adc, None);
        let voltage = adc::VRef::read_vdda(&mut adc);

        let seed = temp * voltage as i16;
        
        // Random Numbers Generator
        let mut rng = SmallRng::seed_from_u64(seed as u64);    
      

        loop {

        }

    });


    loop {
        
        // HERE I CANNOT ACCESS MY PERIPHERALS, UNLESS...


        static mut DISPLAY: Option<ssd1306::mode::terminal::TerminalMode<ssd1306::interface::i2c::I2cInterface
                                  <hal::i2c::I2c<hal::stm32::I2C1,hal::gpio::gpioa::PA9<hal::gpio::Alternate<hal::gpio::AF4>>,
                                  hal::gpio::gpioa::PA10<hal::gpio::Alternate<hal::gpio::AF4>>>>>> = None;
    
        let disp = DISPLAY.get_or_insert_with(|| {
            cortex_m::interrupt::free(|cs| {
                // Move DISPLAY here, leaving a None in its place
                GDISPLAY.borrow(cs).replace(None).unwrap()
            })
        });

        //get the texts from the external module
        let texts = answers();

        let idx = free(|cs| INDEX.borrow(cs).get());

        //let answers = free(|cs| ANSWERS.borrow(cs).get());

        let choice = texts[idx as usize];

        disp.write_str(choice);
        
        //some delay

        continue;}

}

#[interrupt]

fn TIM3() {
        
    static mut TIMER: Option<Timer<TIM3>> = None;
    static mut BTN: Option<PA0<Input<PullUp>>> = None;

    let int = TIMER.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| {
            // Move TIMER here, leaving a None in its place
            GTIMER.borrow(cs).replace(None).unwrap()
        })
    });
    
    let button = BTN.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| {
            // Move BTN here, leaving a None in its place
            GBUTTON.borrow(cs).replace(None).unwrap()
        })
    });
   
    let state1 = free(|cs| STATE1.borrow(cs).get());
    let state2 = free(|cs| STATE2.borrow(cs).get());
    
    let current = button.is_high().unwrap(); //button pressed?

    // HERE THE NEW INDEX VALUE MUST BE GENERATED

    let random_value: u8 = 19_u8;

    if (current == false) && (state1 == true) && (state2 == true) { //if button NOT pressed and both previous states were true
        free(|cs| INDEX.borrow(cs).replace(random_value)); //change the random value
        }
        
    free(|cs| STATE1.borrow(cs).replace(state2)); //shift the previous state into the past
    free(|cs| STATE2.borrow(cs).replace(current));


    //int.wait().ok();

}

