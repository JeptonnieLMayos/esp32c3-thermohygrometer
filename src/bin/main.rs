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
// use fugit::RateExtU32;

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
    i2c = i2c.with_sda(peripherals.GPIO3);
    i2c = i2c.with_scl(peripherals.GPIO4);
    let addr = 0x44;
    let cmd = [0x24, 0x00];
    let mut data = [0u8; 6];
    
    loop
    {
        let res = i2c.write(addr, &cmd);
        println!("{:?}", res);

        led.set_low();
        delay.delay_millis(20);

        i2c.read(addr, &mut data).unwrap();
        let temp_raw = ((data[0] as u16) << 8) | data[1] as u16;
        let hum_raw = ((data[3] as u16) << 8) | data[4] as u16;
        let temperature = -45.0 +175.0 * (temp_raw as f32 / 65535.0);
        let humidity = 100.0 * (hum_raw as f32 / 65535.0);
        println!("Temp: {:.2} C", temperature);
        println!("Humidity: {:.2} %", humidity);
        led.set_high();
        delay.delay_millis(980);
    }
}