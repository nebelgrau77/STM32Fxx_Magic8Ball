//! magic 8-ball prototype
//! 
//! TO DO: 
//! - get a random answer at a press of a button, display it for some time and then clear screen
//! - get the answer to scroll across the screen
//! - store the answers in a separate file for a more modular design

#![no_std]
#![no_main]


extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_halt;
extern crate stm32f0xx_hal as hal;

use cortex_m_rt::entry;
use ssd1306::{prelude::*, Builder as SSD1306Builder};

use embedded_graphics::{
    prelude::*,
    fonts::{Font6x8, Text},
    style::TextStyleBuilder,
    pixelcolor::BinaryColor,
};

use crate::hal::{
    prelude::*,
    stm32,
    delay::Delay,
    i2c::I2c,
    adc,
};

use rand::prelude::*;

// access the answers from a separate module
mod magicball;
use crate::magicball::answers::answers;

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

        //configure the display and set up text style
        let mut disp: GraphicsMode<_> = SSD1306Builder::new().size(DisplaySize::Display128x64).connect_i2c(i2c).into();

        disp.init().unwrap();

        let text_style = TextStyleBuilder::new(Font6x8).text_color(BinaryColor::On).build();

        // get the internal temperature and voltage values to create a randomized initial seed for the Random Numbers Generator

        let temp = adc::VTemp::read(&mut adc, None);
        let voltage = adc::VRef::read_vdda(&mut adc);

        let seed = temp * voltage as i16;
        
        // Random Numbers Generator
        let mut rng = SmallRng::seed_from_u64(seed as u64);    

        //get the texts from the external module
        let texts = answers();
        
        loop {

            // clean up the screen
            for x in 0..128 {
                for y in 0..16 {
                    disp.set_pixel(x,y,0);
                }
            }

            // choose a random answer with an index between 0 and 19
            let mut choice = rng.next_u32();
            choice = choice % 20;

            let text = texts[choice as usize];

            Text::new(text, Point::new(0,0)).into_styled(text_style).draw(&mut disp);

            disp.flush().unwrap();

            delay.delay_ms(1000_u16);

        }

    });


    loop {continue;}

}