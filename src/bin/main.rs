#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_backtrace as _;
use esp_hal::Blocking;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::i2c::master::{Config as I2cConfig, I2c};
use esp_hal::main;
use esp_hal::rmt::Rmt;
use esp_hal::time::Rate;
use esp_hal::time::{Duration, Instant};
use esp_hal_smartled::{SmartLedsAdapter, smart_led_buffer};
use log::{info, warn};
use smart_leds::{RGB8, SmartLedsWrite, brightness, gamma};

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

const IMU_ADDR: u8 = 0x68;
const SHTC3_ADDR: u8 = 0x70;

const IMU_WHO_AM_I: u8 = 0x75;
const IMU_PWR_MGMT0: u8 = 0x1F;
const IMU_ACCEL_CONFIG0: u8 = 0x21;
const IMU_ACCEL_DATA_X1: u8 = 0x0B;

const IMU_DEVICE_ID_ICM42607: u8 = 0x60;
const IMU_DEVICE_ID_ICM42670: u8 = 0x67;

const IMU_ACCEL_G2_50HZ: u8 = 0x60 | 0x0A;
const IMU_ACCEL_LOW_NOISE: u8 = 0x03;
const ACCEL_1G_RAW: i16 = 16_384;
const RGB_MAX_BRIGHTNESS: u8 = 255;
const RGB_LEVEL: u8 = 4;
const RGB_UPDATE_MS: u64 = 50;
const SENSOR_LOG_MS: u64 = 5_000;

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // generator version: 1.3.0
    // generator parameters: --chip esp32c3 -o log -o esp-backtrace -o vscode

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // GPIO7 drives the small red LED on ESP32-C3-DevKit-RUST-1.
    // Keep it low so the previous blink example no longer flashes it.
    let _red_led_off = Output::new(peripherals.GPIO7, Level::Low, OutputConfig::default());

    let mut delay = Delay::new();

    let mut i2c = I2c::new(
        peripherals.I2C0,
        I2cConfig::default().with_frequency(Rate::from_khz(100)),
    )
    .unwrap()
    .with_sda(peripherals.GPIO10)
    .with_scl(peripherals.GPIO8);

    if let Err(err) = init_imu(&mut i2c, &mut delay) {
        warn!("ICM42670 init failed: {:?}", err);
    }

    if let Err(err) = reset_shtc3(&mut i2c, &mut delay) {
        warn!("SHTC3 reset failed: {:?}", err);
    }

    let rmt: Rmt<'_, Blocking> = Rmt::new(peripherals.RMT, Rate::from_mhz(80)).unwrap();
    let mut rmt_buffer = smart_led_buffer!(1);
    let mut rgb_led = SmartLedsAdapter::new(rmt.channel0, peripherals.GPIO2, &mut rmt_buffer);

    if let Err(err) = write_rgb(&mut rgb_led, RGB8::new(0, 0, 0)) {
        warn!("RGB initial off failed: {:?}", err);
    }

    let mut last_rgb_update = Instant::now();
    let mut last_sensor_log = Instant::now();
    let mut last_accel = Accel::default();

    loop {
        let now = Instant::now();

        if last_rgb_update.elapsed() >= Duration::from_millis(RGB_UPDATE_MS) {
            last_rgb_update = now;

            match read_accel(&mut i2c) {
                Ok(accel) => {
                    last_accel = accel;
                    let color = accel_to_rgb(accel);

                    if let Err(err) = write_rgb(&mut rgb_led, color) {
                        warn!("RGB write failed: {:?}", err);
                    }
                }
                Err(err) => warn!("ICM42670 accel read failed: {:?}", err),
            }
        }

        if last_sensor_log.elapsed() >= Duration::from_millis(SENSOR_LOG_MS) {
            last_sensor_log = now;

            match read_shtc3(&mut i2c, &mut delay) {
                Ok(climate) => info!(
                    "SHTC3 temp={:.2} C humidity={:.2}% accel_raw=({}, {}, {}) rgb=({}, {}, {})",
                    climate.temperature_c,
                    climate.humidity_percent,
                    last_accel.x,
                    last_accel.y,
                    last_accel.z,
                    accel_to_rgb(last_accel).r,
                    accel_to_rgb(last_accel).g,
                    accel_to_rgb(last_accel).b
                ),
                Err(err) => warn!("SHTC3 read failed: {:?}", err),
            }
        }
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.1.0/examples
}

#[derive(Debug)]
enum SensorError<E> {
    I2c(E),
    Crc,
    BadDeviceId,
}

#[derive(Clone, Copy, Debug, Default)]
struct Accel {
    x: i16,
    y: i16,
    z: i16,
}

#[derive(Clone, Copy, Debug)]
struct Climate {
    temperature_c: f32,
    humidity_percent: f32,
}

fn init_imu<I2C>(i2c: &mut I2C, delay: &mut Delay) -> Result<(), SensorError<I2C::Error>>
where
    I2C: embedded_hal::i2c::I2c,
{
    let device_id = read_reg(i2c, IMU_ADDR, IMU_WHO_AM_I).map_err(SensorError::I2c)?;

    if device_id != IMU_DEVICE_ID_ICM42607 && device_id != IMU_DEVICE_ID_ICM42670 {
        warn!("unexpected ICM42670 WHO_AM_I=0x{:02X}", device_id);
        return Err(SensorError::BadDeviceId);
    }

    write_reg(i2c, IMU_ADDR, IMU_ACCEL_CONFIG0, IMU_ACCEL_G2_50HZ).map_err(SensorError::I2c)?;
    write_reg(i2c, IMU_ADDR, IMU_PWR_MGMT0, IMU_ACCEL_LOW_NOISE).map_err(SensorError::I2c)?;
    delay.delay_millis(50);

    info!("ICM42670 ready, device_id=0x{:02X}", device_id);

    Ok(())
}

fn read_accel<I2C>(i2c: &mut I2C) -> Result<Accel, SensorError<I2C::Error>>
where
    I2C: embedded_hal::i2c::I2c,
{
    let mut data = [0_u8; 6];
    i2c.write_read(IMU_ADDR, &[IMU_ACCEL_DATA_X1], &mut data)
        .map_err(SensorError::I2c)?;

    Ok(Accel {
        x: i16::from_be_bytes([data[0], data[1]]),
        y: i16::from_be_bytes([data[2], data[3]]),
        z: i16::from_be_bytes([data[4], data[5]]),
    })
}

fn accel_to_rgb(accel: Accel) -> RGB8 {
    RGB8 {
        r: axis_to_brightness(accel.x),
        g: axis_to_brightness(accel.y),
        b: axis_to_brightness(accel.z),
    }
}

fn write_rgb<const BUFFER_SIZE: usize>(
    rgb_led: &mut SmartLedsAdapter<'_, BUFFER_SIZE>,
    color: RGB8,
) -> Result<(), esp_hal_smartled::LedAdapterError> {
    rgb_led.write(brightness(gamma([color].into_iter()), RGB_LEVEL))
}

fn axis_to_brightness(value: i16) -> u8 {
    let magnitude = value.unsigned_abs().min(ACCEL_1G_RAW as u16) as u32;
    ((magnitude * RGB_MAX_BRIGHTNESS as u32) / ACCEL_1G_RAW as u32) as u8
}

fn reset_shtc3<I2C>(i2c: &mut I2C, delay: &mut Delay) -> Result<(), SensorError<I2C::Error>>
where
    I2C: embedded_hal::i2c::I2c,
{
    i2c.write(SHTC3_ADDR, &[0x80, 0x5D])
        .map_err(SensorError::I2c)?;
    delay.delay_millis(1);
    Ok(())
}

fn read_shtc3<I2C>(i2c: &mut I2C, delay: &mut Delay) -> Result<Climate, SensorError<I2C::Error>>
where
    I2C: embedded_hal::i2c::I2c,
{
    i2c.write(SHTC3_ADDR, &[0x78, 0x66])
        .map_err(SensorError::I2c)?;
    delay.delay_millis(12);

    let mut data = [0_u8; 6];
    i2c.read(SHTC3_ADDR, &mut data).map_err(SensorError::I2c)?;

    if crc8(&data[0..2]) != data[2] || crc8(&data[3..5]) != data[5] {
        return Err(SensorError::Crc);
    }

    let raw_temperature = u16::from_be_bytes([data[0], data[1]]) as u32;
    let raw_humidity = u16::from_be_bytes([data[3], data[4]]) as u32;

    let temperature_millic = ((raw_temperature * 21_875) >> 13) as i32 - 45_000;
    let humidity_millipercent = ((raw_humidity * 12_500) >> 13) as i32;

    Ok(Climate {
        temperature_c: temperature_millic as f32 / 1_000.0,
        humidity_percent: humidity_millipercent as f32 / 1_000.0,
    })
}

fn read_reg<I2C>(i2c: &mut I2C, address: u8, register: u8) -> Result<u8, I2C::Error>
where
    I2C: embedded_hal::i2c::I2c,
{
    let mut data = [0_u8; 1];
    i2c.write_read(address, &[register], &mut data)?;
    Ok(data[0])
}

fn write_reg<I2C>(i2c: &mut I2C, address: u8, register: u8, value: u8) -> Result<(), I2C::Error>
where
    I2C: embedded_hal::i2c::I2c,
{
    i2c.write(address, &[register, value])
}

fn crc8(data: &[u8]) -> u8 {
    let mut crc = 0xFF_u8;

    for byte in data {
        crc ^= *byte;
        for _ in 0..8 {
            crc = if crc & 0x80 != 0 {
                (crc << 1) ^ 0x31
            } else {
                crc << 1
            };
        }
    }

    crc
}
