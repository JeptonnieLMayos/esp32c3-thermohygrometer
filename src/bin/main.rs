#![no_std]
#![no_main]

use esp_println::println;
use esp_hal::
{
    clock::CpuClock,
    delay::Delay,
    gpio::
    {
        Level, Output, OutputConfig
    },
    i2c::master::
    {
        I2c,
        Config
    },
    time::Rate,
};

use ssd1306::
{
    prelude::*,
    Ssd1306,
    I2CDisplayInterface,
};
use embedded_hal::i2c::I2c as _;
use embedded_hal_bus::i2c::RefCellDevice;
use core::cell::RefCell;

use embedded_graphics::{
    prelude::*,
    text::Text,
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
};
use heapless::String;
use core::fmt::Write;


#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[esp_hal::main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let mut led = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());

    let delay = Delay::new();

    let i2c_config = Config::default().with_frequency(Rate::from_khz(100));
    let mut i2c = I2c::new(peripherals.I2C0, i2c_config).unwrap();
    i2c = i2c.with_sda(peripherals.GPIO3)
            .with_scl(peripherals.GPIO4);


    let addr = 0x44;
    let cmd = [0x24, 0x00];
    let mut data = [0u8; 6];


    let i2c_bus = RefCell::new(i2c);
    let interface = I2CDisplayInterface::new(
        RefCellDevice::new(&i2c_bus)
    );



    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    )
    .into_buffered_graphics_mode();
    display.init().unwrap();
    display.flush().unwrap();

    let text_style = MonoTextStyleBuilder::new()
    .font(&FONT_6X10)
    .text_color(BinaryColor::On)
    .build();



    
    loop
    {
        display.clear_buffer();
        let mut i2c = RefCellDevice::new(&i2c_bus);
        i2c.write(addr, &cmd).unwrap();
        

        led.set_low();
        delay.delay_millis(20);
        
        display.clear_buffer();
        
        i2c.read(addr, &mut data).unwrap();
        let temp_raw = ((data[0] as u16) << 8) | data[1] as u16;
        let hum_raw = ((data[3] as u16) << 8) | data[4] as u16;
        let temperature = -45.0 +175.0 * (temp_raw as f32 / 65535.0);
        let humidity = 100.0 * (hum_raw as f32 / 65535.0);
        println!("Temp: {:.2} C", temperature);
        println!("Humidity: {:.2} %", humidity);
        let mut line1: String<32> = String::new();
        let mut line2: String<32> = String::new();
        write!(line1, "{:.1}C", temperature).unwrap();
        write!(line2, "{:.1}%", humidity).unwrap();

        Text::new(&line1, Point::new(0,16), text_style)
            .draw(&mut display)
            .unwrap();

        Text::new(&line2, Point::new(0,32), text_style)
            .draw(&mut display)
            .unwrap();

        display.flush().unwrap();

        led.set_high();
        delay.delay_millis(2980);
    }
}