# Requirements
Before you use Laptop-Toio, you will need to download:
- [Processing](https://processing.org/download)
- [Rust](https://www.rust-lang.org/tools/install)

# Structure
![Frame 1](https://github.com/AxLab-UofC/Laptop-TOIO/assets/66401951/841f6990-a5f2-4c6b-8e87-c6a39817b4cd)

All of the Low-Level Bluetooth Toio is handled within Rust, while the Higher-Level Simulations is done within Processing.

# Connecting with Rust
Our Rust code allows us to connect to toios by using their 'name' (An internal 3-digit alphanumeric id built into the firmware), or their AxLab ID (Written on the bottom of the toio). We can connect using the following command

```
cargo run -- -a 48,46
```

NOTE: Do not uses spaces between the IDs.

Using the `-a` flag allows us to connect using the AxLab ID. When needed, we can also use `-n` to connect directly to their internal name, or even combine the two. For example: `cargo run -- -n P1B` or `cargo run -- -a 46 -n P1B`

## Special Flags

There are special flags for specific cases.

- Running Laptop-Toio with the command `cargo run -- -s` will allow you to connect to all nearby toios, in case you don't want to deal with the hassle of looking over which toios you have. NOTE: Only use this flag when there is no one else using toios nearby.
- Running Laptop-Toio with the command `cargo run -- -o -a 48,46` will attempt to connect to the toios in the specific order you passed them in. In this case, that would be 48 and then 46


# Running the Simulation with Processing
The full [toio API](https://toio.github.io/toio-spec/en/docs/about) allows us to send commands and request information from the toio. Most of these API requests have been implemented within Laptop-toio on the Rust and Processing side to allow us to control toios. All of these functions are available on the `Cube` tab of the processing code.

We are send comannds to control to control:
- [The Motors](https://toio.github.io/toio-spec/en/docs/ble_motor):
- [The LED Indicator](https://toio.github.io/toio-spec/en/docs/ble_light):
- [The Speaker](https://toio.github.io/toio-spec/en/docs/ble_sound):

We automatically recieve updates about:
- [Position](https://toio.github.io/toio-spec/en/docs/ble_id)
- [Battery](https://toio.github.io/toio-spec/en/docs/ble_battery)
- [Button](https://toio.github.io/toio-spec/en/docs/ble_button)

We are able to request information from:
- [Motion Sensor](https://toio.github.io/toio-spec/en/docs/ble_sensor)
- [Posture Angle Sensor](https://toio.github.io/toio-spec/en/docs/ble_high_precision_tilt_sensor)
- [Magnetic Sensor](https://toio.github.io/toio-spec/en/docs/ble_magnetic_sensor)

