use esp_hal::gpio::{Output, Input};
use esp_hal::delay::Delay;
use esp_hal::time::{Instant, Duration};

pub struct HcSr04 {
    trigger: Output<'static>,
    echo: Input<'static>,
    delay: Delay,
}

impl HcSr04 {
    pub fn new(trigger: Output<'static>, echo: Input<'static>) -> Self {
        Self {
            trigger,
            echo,
            delay: Delay::new(),
        }
    }

    /// Trigger a measurement and return distance in centimeters
    pub fn measure_distance_cm(&mut self) -> f32 {
        // Send 10µs trigger pulse
        self.trigger.set_high();
        self.delay.delay_micros(10); // 10µs
        self.trigger.set_low();

        // Wait for echo to go high (start of echo pulse)
        let timeout = Duration::from_millis(30); // 30ms timeout (~5m max)
        let start = Instant::now();

        // Wait for echo HIGH
        while self.echo.is_low() {
            if start.elapsed() >= timeout {
                return -1.0; // Timeout - no object detected
            }
        }

        let echo_start = Instant::now();

        // Measure echo pulse duration
        while self.echo.is_high() {
            if echo_start.elapsed() >= timeout {
                return -1.0; // Timeout - echo too long
            }
        }

        let echo_duration = echo_start.elapsed();
        let echo_duration_us = echo_duration.as_micros() as f32;

        // Calculate distance: (duration * speed of sound) / 2
        // Speed of sound = 343 m/s = 0.0343 cm/µs
        // Distance = (duration_us * 0.0343) / 2
        let distance_cm = (echo_duration_us * 0.0343) / 2.0;

        distance_cm
    }

    /// Measure distance with multiple samples for accuracy
    pub fn measure_distance_avg(&mut self, samples: u8) -> f32 {
        let mut total = 0.0f32;
        let mut valid_samples = 0u8;

        for _ in 0..samples {
            let distance = self.measure_distance_cm();
            if distance > 0.0 {
                total += distance;
                valid_samples += 1;
            }
            self.delay.delay_millis(60); // Minimum 60ms between measurements
        }

        if valid_samples > 0 {
            total / valid_samples as f32
        } else {
            -1.0
        }
    }
}
