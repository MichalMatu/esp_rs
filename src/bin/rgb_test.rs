#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types"
)]
#![deny(clippy::large_stack_frames)]

use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use esp_hal::rmt::Rmt;
use esp_hal::time::Rate;
use esp_hal_smartled::{SmartLedsAdapter, smart_led_buffer};
use smart_leds::{RGB8, SmartLedsWrite, brightness, gamma};

esp_bootloader_esp_idf::esp_app_desc!();

const RGB_LEVEL: u8 = 4;

#[allow(
    clippy::large_stack_frames,
    reason = "it's acceptable for the top-level embedded entry point"
)]
#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let _red_led_off = Output::new(peripherals.GPIO7, Level::Low, OutputConfig::default());

    let rmt: Rmt<'_, esp_hal::Blocking> = Rmt::new(peripherals.RMT, Rate::from_mhz(80)).unwrap();
    let mut rmt_buffer = smart_led_buffer!(1);
    let mut rgb = SmartLedsAdapter::new(rmt.channel0, peripherals.GPIO2, &mut rmt_buffer);
    let delay = Delay::new();

    loop {
        write_one(&mut rgb, RGB8::new(16, 0, 0));
        delay.delay_millis(800);
        write_one(&mut rgb, RGB8::new(0, 16, 0));
        delay.delay_millis(800);
        write_one(&mut rgb, RGB8::new(0, 0, 16));
        delay.delay_millis(800);
        write_one(&mut rgb, RGB8::new(0, 0, 0));
        delay.delay_millis(800);
    }
}

fn write_one<const BUFFER_SIZE: usize>(rgb: &mut SmartLedsAdapter<'_, BUFFER_SIZE>, color: RGB8) {
    rgb.write(brightness(gamma([color].into_iter()), RGB_LEVEL))
        .unwrap();
}
