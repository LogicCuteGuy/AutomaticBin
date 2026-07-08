use esp_hal::ledc::{channel, LowSpeed};
use esp_hal::ledc::channel::ChannelIFace;

pub struct Servo<'a> {
    channel: channel::Channel<'a, LowSpeed>,
}

impl<'a> Servo<'a> {
    pub fn new(channel: channel::Channel<'a, LowSpeed>) -> Self {
        Self { channel }
    }

    /// Set servo angle (0-180 degrees)
    pub fn set_angle(&mut self, angle: u8) {
        // MG90S: 1ms (0°) to 2ms (180°) at 50Hz (20ms period)
        // Duty cycle percentage = (pulse_width_us / period_us) * 100
        // For 1ms: 1000/20000 * 100 = 5%
        // For 2ms: 2000/20000 * 100 = 10%

        let angle = angle.min(180);
        let pulse_width_us = 1000 + (angle as u32 * 1000 / 180); // 1000-2000µs
        let duty_pct = (pulse_width_us * 100 / 20000) as u8; // 5-10%

        self.channel.set_duty(duty_pct).ok();
    }

    /// Initialize servo to center position (90°)
    pub fn center(&mut self) {
        self.set_angle(90);
    }

    /// Sweep servo through range
    pub fn sweep(&mut self, min_angle: u8, max_angle: u8, step: u8, delay_ms: u32) {
        let delay = esp_hal::delay::Delay::new();

        // Sweep up
        for angle in (min_angle..=max_angle).step_by(step as usize) {
            self.set_angle(angle);
            delay.delay_millis(delay_ms);
        }

        // Sweep down
        for angle in (min_angle..=max_angle).rev().step_by(step as usize) {
            self.set_angle(angle);
            delay.delay_millis(delay_ms);
        }
    }
}

/// Dual servo controller for bin lid
pub struct BinServos<'a> {
    pub left: Servo<'a>,
    pub right: Servo<'a>,
}

impl<'a> BinServos<'a> {
    pub fn new(left: Servo<'a>, right: Servo<'a>) -> Self {
        Self { left, right }
    }

    /// Open bin lid (both servos to 180°)
    pub fn open(&mut self) {
        self.left.set_angle(180);
        self.right.set_angle(180);
    }

    /// Close bin lid (both servos to 0°)
    pub fn close(&mut self) {
        self.left.set_angle(0);
        self.right.set_angle(0);
    }

    /// Set both servos to specific angle
    pub fn set_both(&mut self, angle: u8) {
        self.left.set_angle(angle);
        self.right.set_angle(angle);
    }
}
