#![no_std]
#![no_main]

esp_bootloader_esp_idf::esp_app_desc!();

use esp_backtrace as _;
use esp_hal::{delay::Delay, gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull, DriveMode}, ledc::{timer, channel, LowSpeed, Ledc, LSGlobalClkSource}, time::Rate};
use esp_hal::ledc::timer::TimerIFace;
use esp_hal::ledc::channel::ChannelIFace;
use esp_println::println;
use defmt_rtt as _;
use esp_hal::xtensa_lx_rt::entry;

use automatic_bin::hc_sr04::HcSr04;
use automatic_bin::servo::{Servo, BinServos};

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Initialize LED (D2 - built-in)
    let mut led = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());

    // Initialize HC-SR04 pins
    let trig_pin = Output::new(peripherals.GPIO32, Level::Low, OutputConfig::default());
    let echo_pin = Input::new(peripherals.GPIO34, InputConfig::default().with_pull(Pull::Down));

    // Initialize servo pins
    let servo1_pin = Output::new(peripherals.GPIO18, Level::Low, OutputConfig::default());
    let servo2_pin = Output::new(peripherals.GPIO19, Level::Low, OutputConfig::default());

    // Initialize LEDC for PWM
    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let mut timer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    timer0.configure(timer::config::Config {
        frequency: Rate::from_hz(50),
        duty: timer::config::Duty::Duty13Bit,
        clock_source: timer::LSClockSource::APBClk,
    }).unwrap();

    // Create and configure servo channels
    let mut channel0 = ledc.channel(channel::Number::Channel0, servo1_pin);
    channel0.configure(channel::config::Config {
        timer: &timer0,
        duty_pct: 0,
        drive_mode: DriveMode::PushPull,
    }).unwrap();

    let mut channel1 = ledc.channel(channel::Number::Channel1, servo2_pin);
    channel1.configure(channel::config::Config {
        timer: &timer0,
        duty_pct: 0,
        drive_mode: DriveMode::PushPull,
    }).unwrap();

    // Create servo instances
    let servo1 = Servo::new(channel0);
    let servo2 = Servo::new(channel1);

    // Create HC-SR04 instance
    let mut sensor = HcSr04::new(trig_pin, echo_pin);

    // Create bin servos controller
    let mut bin_servos = BinServos::new(servo1, servo2);

    // Threshold distance in centimeters
    const THRESHOLD_CM: f32 = 30.0;

    // Indicator that system is ready
    println!("Automatic Bin System Ready!");
    println!("Threshold: {} cm", THRESHOLD_CM);

    // Blink LED to indicate ready state
    let delay = Delay::new();
    for _ in 0..5 {
        led.set_high();
        delay.delay_millis(200);
        led.set_low();
        delay.delay_millis(200);
    }

    // Main loop
    loop {
        // Measure distance
        let distance = sensor.measure_distance_avg(3);

        if distance > 0.0 {
            println!("Distance: {:.2} cm", distance);

            if distance < THRESHOLD_CM {
                // Object detected - open bin
                println!("Opening bin - object detected!");
                bin_servos.open();
                led.set_high();

                // Keep lid open while object is present
                loop {
                    let dist = sensor.measure_distance_avg(2);
                    if dist > THRESHOLD_CM || dist < 0.0 {
                        break;
                    }
                    delay.delay_millis(100);
                }

                // Close bin
                println!("Closing bin - object removed");
                bin_servos.close();
                led.set_low();
            }
        }

        // Small delay before next measurement
        delay.delay_millis(50);
    }
}
