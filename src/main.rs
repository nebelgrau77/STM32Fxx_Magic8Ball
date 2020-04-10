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
};

mod magicball;

use crate::magicball::answers::answers;

use rand::prelude::*;

const BOOT_DELAY_MS: u16 = 200;

#[entry]

fn main() -> ! {
    
    let mut p = stm32::Peripherals::take().unwrap();
    let mut cp = cortex_m::peripheral::Peripherals::take().unwrap();

    cortex_m::interrupt::free(move |cs| {

        let mut rcc = p.RCC.configure().sysclk(8.mhz()).freeze(&mut p.FLASH);

        let mut delay = Delay::new(cp.SYST, &rcc);

        delay.delay_ms(BOOT_DELAY_MS);

        let gpioa = p.GPIOA.split(&mut rcc);
        let scl = gpioa.pa9.into_alternate_af4(cs);
        let sda = gpioa.pa10.into_alternate_af4(cs);
        let i2c = I2c::i2c1(p.I2C1, (scl, sda), 400.khz(), &mut rcc);

        let mut disp: GraphicsMode<_> = SSD1306Builder::new().size(DisplaySize::Display128x64).connect_i2c(i2c).into();

        disp.init().unwrap();

        let text_style = TextStyleBuilder::new(Font6x8).text_color(BinaryColor::On).build();

        let mut rng = SmallRng::seed_from_u64(0x0101_0303_0808_0909);    
        
        let texts = answers();
        
        loop {

            for x in 0..128 {
                for y in 0..64 {
                    disp.set_pixel(x,y,0);
                }
            }


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