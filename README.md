# AutomaticBin - ESP32 Rust Project

Automatic trash bin controller using HC-SR04 ultrasonic sensor and MG90S servo motors, written in Rust for ESP32.

## Features

- **Ultrasonic Distance Sensing**: HC-SR04 sensor measures distance (2cm - 400cm)
- **Servo Control**: 2x MG90S servo motors for bin lid actuation
- **Threshold Trigger**: Opens bin when object detected within 30cm
- **Auto-Close**: Closes bin lid when object is removed
- **LED Indicator**: Built-in LED (D2) indicates system ready state

## Hardware Requirements

### Components
- ESP32 development board
- HC-SR04 ultrasonic sensor
- 2x MG90S servo motors
- LEDs (optional, for external indicators)
- Jumper wires
- Power supply (5V for servos and sensor)

### Pin Connections

```
HC-SR04 Ultrasonic Sensor:
┌─────────────────────────────────────────┐
│  VCC  → 5V                             │
│  GND  → GND                            │
│  Trig → GPIO 32 (D32)                  │
│  Echo → GPIO 34 (D34)                  │
└─────────────────────────────────────────┘

MG90S Servo Motors:
┌─────────────────────────────────────────┐
│  Servo 1:                              │
│    Signal → GPIO 18 (D18)              │
│    VCC    → 5V                         │
│    GND    → GND                        │
│                                         │
│  Servo 2:                              │
│    Signal → GPIO 19 (D19)              │
│    VCC    → 5V                         │
│    GND    → GND                        │
└─────────────────────────────────────────┘

Built-in LED:
┌─────────────────────────────────────────┐
│  GPIO 2 (D2) - Built-in LED            │
│  Indicates system ready state          │
└─────────────────────────────────────────┘
```

## Prerequisites

### 1. Install Rust ESP Toolchain

```bash
# Install espup
cargo install espup

# Install ESP32 toolchain
espup install

# Source environment (run in each terminal session)
source ~/export-esp.sh
```

### 2. Install ESP Flash Tool

```bash
cargo install espflash
```

### 3. Verify Installation

```bash
# Check Rust toolchain
rustup show

# Should show 'esp' toolchain installed
# Check espflash
espflash --version
```

## Building the Project

### Debug Build (Development)

```bash
# Source ESP environment
source ~/export-esp.sh

# Build debug version (faster compilation, larger binary)
cargo build
```

### Release Build (Production)

```bash
# Source ESP environment
source ~/export-esp.sh

# Build optimized release version (slower compilation, smaller binary)
cargo build --release
```

### Build Output Location

- **Debug**: `target/xtensa-esp32-none-elf/debug/automatic_bin`
- **Release**: `target/xtensa-esp32-none-elf/release/automatic_bin`

## Flashing to ESP32

### Method 1: Using espflash (Recommended)

```bash
# Source ESP environment
source ~/export-esp.sh

# Build and flash in one command
cargo run --release

# Or flash pre-built binary
espflash flash --monitor target/xtensa-esp32-none-elf/release/automatic_bin
```

### Method 2: Manual Flash

```bash
# Build first
cargo build --release

# Flash with espflash
espflash flash target/xtensa-esp32-none-elf/release/automatic_bin

# Monitor serial output (optional)
espflash monitor
```

### Method 3: Using cargo-espflash

```bash
# Install cargo-espflash
cargo install cargo-espflash

# Flash and monitor
cargo espflash --release --monitor
```

## Monitoring Serial Output

```bash
# Using espflash
espflash monitor

# Using idf-monitor (if ESP-IDF installed)
idf.py monitor

# Baud rate: 115200
```

## Project Structure

```
AutomaticBin/
├── Cargo.toml              # Project configuration
├── rust-toolchain.toml     # Rust toolchain config
├── .cargo/
│   └── config.toml         # Build configuration
├── src/
│   ├── lib.rs              # Library root
│   ├── hc_sr04.rs          # HC-SR04 sensor driver
│   ├── servo.rs            # Servo motor controller
│   └── bin/
│       └── main.rs         # Main application
└── README.md               # This file
```

## Configuration

### Threshold Distance

Edit `src/bin/main.rs` to change the threshold distance:

```rust
// Threshold distance in centimeters
const THRESHOLD_CM: f32 = 30.0;  // Change this value
```

### Pin Assignments

Edit `src/bin/main.rs` to change pin assignments:

```rust
// GPIO pins
let trig_pin = Output::new(peripherals.GPIO32, ...);  // Change GPIO32
let echo_pin = Input::new(peripherals.GPIO34, ...);   // Change GPIO34
let servo1_pin = Output::new(peripherals.GPIO18, ...); // Change GPIO18
let servo2_pin = Output::new(peripherals.GPIO19, ...); // Change GPIO19
```

### Servo Angles

Edit `src/servo.rs` to adjust servo angles:

```rust
/// Open bin lid (both servos to 180°)
pub fn open(&mut self) {
    self.left.set_angle(180);   // Adjust angle
    self.right.set_angle(180);  // Adjust angle
}

/// Close bin lid (both servos to 0°)
pub fn close(&mut self) {
    self.left.set_angle(0);     // Adjust angle
    self.right.set_angle(0);    // Adjust angle
}
```

## Troubleshooting

### Build Errors

1. **"can't find crate for `core`"**
   ```bash
   # Ensure you've sourced the ESP environment
   source ~/export-esp.sh
   ```

2. **"undefined reference to `_stack_end_cpu0`"**
   - Check `.cargo/config.toml` has correct linker arguments
   - Ensure `linkall.x` is included in rustflags

3. **"no method named `Hertz` found"**
   - Use `Rate::from_hz(50)` instead of `50u32.Hertz()`

### Flash Errors

1. **"Could not auto-detect serial port"**
   - Connect ESP32 via USB
   - Press and hold BOOT button while flashing
   - Try: `espflash flash --port /dev/ttyUSB0`

2. **"Failed to connect to ESP32"**
   - Press BOOT button during flash
   - Check USB cable supports data (not charge-only)
   - Try different USB port

### Runtime Issues

1. **LED not blinking**
   - Check GPIO2 connection (built-in LED)
   - Verify power supply

2. **Servos not moving**
   - Ensure 5V power supply for servos
   - Check signal wire connections
   - Verify servo angles in code

3. **Distance readings incorrect**
   - Check HC-SR04 wiring
   - Ensure sensor has clear line of sight
   - Check for interference from other objects

## API Reference

### HC-SR04 Sensor

```rust
// Create sensor instance
let mut sensor = HcSr04::new(trig_pin, echo_pin);

// Single measurement (returns distance in cm, -1.0 on timeout)
let distance = sensor.measure_distance_cm();

// Average of multiple samples
let distance = sensor.measure_distance_avg(3);
```

### Servo Control

```rust
// Create servo instance
let mut servo = Servo::new(channel);

// Set angle (0-180 degrees)
servo.set_angle(90);

// Center position
servo.center();

// Sweep through range
servo.sweep(0, 180, 5, 100);  // min, max, step, delay_ms
```

### Bin Servos

```rust
// Create dual servo controller
let mut bin_servos = BinServos::new(servo1, servo2);

// Open bin lid
bin_servos.open();

// Close bin lid
bin_servos.close();

// Set both servos to angle
bin_servos.set_both(90);
```

## Development

### Code Quality

```bash
# Run clippy linter
cargo clippy

# Format code
cargo fmt

# Check formatting
cargo fmt --check
```

### Testing

```bash
# Run tests (if any)
cargo test
```

### Documentation

```bash
# Generate documentation
cargo doc --open
```

## License

This project is open source and available under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Resources

- [ESP-HAL Documentation](https://docs.espressif.com/projects/rust/esp-hal/latest/)
- [Rust on ESP Book](https://docs.espressif.com/projects/rust/book/)
- [ESP-RS GitHub](https://github.com/esp-rs)
- [HC-SR04 Datasheet](https://cdn.sparkfun.com/datasheets/Sensors/HC-SR04.pdf)
- [MG90S Servo Datasheet](https://www.servodatabase.com/servo/tower-pro/mg-90s)

## Support

For issues and questions:
- Check the troubleshooting section above
- Open an issue on GitHub
- Join the ESP-RS community on Matrix/Discord
