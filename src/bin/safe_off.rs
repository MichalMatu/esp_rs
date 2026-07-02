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

esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's acceptable for the top-level embedded entry point"
)]
#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let _rgb_data_low = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());
    let _red_led_off = Output::new(peripherals.GPIO7, Level::Low, OutputConfig::default());
    let delay = Delay::new();

    loop {
        delay.delay_millis(1_000);
    }
}
