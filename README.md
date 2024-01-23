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
The full [toio API](https://toio.github.io/toio-spec/en/docs/about) allows us to send commands and request information from the toio. Most of these API requests have been implemented within Laptop-toio on the Rust and Processing side to allow us to control toios. All of these functions are available on the `Cube` tab of the processing code. All of the toios are stored an array called `cubes`. This means that you can access each toio with `cubes[i]`, where `i` is the order thetoios connected in.

## Processing Code Structure
There are 5 tabs in the Laptop-Toio Processing Code:
- The `toio processing` tab includes three core functions: 
    - `settings`: The function allows you to configure your program
    - `setup`: This function is called once at the beginning of your program
    - `draw`: This function is called continously every tick
This tab is analogous to the `setup` and `loop` structure in arduino
- The `cube` tab includes all of the information on the `Cube` struct which stores most of the values and methods used to communicate with toios
- The `events` tab allows you to handle different events from the keyboard, mouse or the toios.
- The `motorVelocityTarget` tab includes the code for the `velocityTarget` function, which is an advanced function used to travel smoothly over time
- The `osc` tab handles the communication with the rust code, and sends commands to the toios.

For the most part, you will purely be writing your code in the `toio processing` and `events` tabs, and `cube` can be refered for you to quickly understand different commands.

## Writing toio Processing code
Processing allows you to write code in two ways:
- The `draw()` function is called continously on a loop. If you want your code to run on a regular interval, this is the best place to put it.
- The `events` tab is comprised of many functions that are called on certain events. 
    - Keyboard and Mouse events call the `keyPressed`, `mousePressed` and `mouseReleased` functions. If you want an extended look at the GUI events that are registered by Processing, you can look at the Keyboard and Mouse sections [here](https://processing.org/reference/#input).
    - toio events are call the `buttonDown`, `buttonUp`, `collision` and `doubleTap` functions. These will automatically pass the toio IDs as a function parameter, allowing you to use them in your code.

## Writing toio Processing commands

### OUTPUT Commands
We are send comannds to control to control:
- [The Motors](https://toio.github.io/toio-spec/en/docs/ble_motor): NOTE: the maximum speed of a motor on a toio is 115. That commands that can control the toio are:
    -  `cubes[i].motor`: can be called with either `leftSpeed, rightSpeed` or `leftSpeed, rightSpeed, duration`, allowing you to control the speed of each motor individually for a set duration. Further documentation can be found [here](https://toio.github.io/toio-spec/en/docs/ble_motor#motor-control) and [here](https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-specified-duration).
    - `cubes[i].target`: can be called with `x, y, theta`, `mode, x, y, theta` or `control, timeout, mode, maxspeed, speedchange, x, y, theta`, allowing you to target a specific location. Further documentation can be found [here](https://toio.github.io/toio-spec/en/docs/ble_motor#motor-control-with-target-specified).
    - `cubes[i].accelerate`: can be called with `speed, acc, rotateVelocity, rotateDir, dir, priority, duration`, allowing you to control acceleration of the motors. Further documentation can be found [here](https://toio.github.io/toio-spec/en/docs/ble_motor#motor-control-with-acceleration-specified).
    - `cubes[i].multiTarget`: can be called with `mode, targets` or `control, timeout, mode, maxspeed, speedchange, targets` where targets are a 2D array for each target marked as `{x, y}` or `{x, y, theta}`. Unless specified, `theta = 0`. Further documentation can be found [here](https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-multiple-targets-specified).
- [The LED Indicator](https://toio.github.io/toio-spec/en/docs/ble_light):
    - `cubes[i].led` can be used to set the LED to a certain color with `duration, red, green, blue` or to produce an LED sequence with `repetitions, lights` where `lights` are a 2D array with each light in the sequence arranged as `{duration, red, green, blue}`. Further documentation can be found [here](https://toio.github.io/toio-spec/en/docs/ble_light).
- [The Speaker](https://toio.github.io/toio-spec/en/docs/ble_sound):
    - `cubes[i].sound` can be called with `soundeffect, volume` to play sound effects. Further documentation can be found [here](https://toio.github.io/toio-spec/en/docs/ble_sound).
    - `cubes[i].midi` can be used to play a certain note with `duration, noteID, volume` or to produce an note sequence with `repetitions, notes` where `note` are a 2D array with each note in the sequence arranged as `{duration, noteID, volume}` or `{duration, noteID}`. Unless specified, `volume = 255`. Further documentation can be found [here](https://toio.github.io/toio-spec/en/docs/ble_sound/#playing-the-midi-note-numbers).


### INPUT Parameters and Commands

We automatically recieve updates about:
- [Position](https://toio.github.io/toio-spec/en/docs/ble_id): The values of `cubes[i].x`, `cubes[i].y`, and `cubes[i].theta` will automatically update to the location and angle of each toio.
- [Battery](https://toio.github.io/toio-spec/en/docs/ble_battery): The value of `cubes[i].battery` will automatically update to the battery level of each toio.
- [Button](https://toio.github.io/toio-spec/en/docs/ble_button): The value of `cubes[i].buttonDown` will automatically change on button press of each toio. Changes in this state will also trigger the `buttonDown` and `buttonUp` functions.

We are able to request information from:
- [Motion Sensor](https://toio.github.io/toio-spec/en/docs/ble_sensor):
- [Posture Angle Sensor](https://toio.github.io/toio-spec/en/docs/ble_high_precision_tilt_sensor):
- [Magnetic Sensor](https://toio.github.io/toio-spec/en/docs/ble_magnetic_sensor):



