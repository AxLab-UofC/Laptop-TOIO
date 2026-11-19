# Requirements
Before you use Laptop-Toio, you will need to download:
- [Processing](https://processing.org/download)
    - Also make sure to add the `oscP5` library from the `Sketch > Import Library... > Manage Libraries...` button.
- [Rust](https://www.rust-lang.org/tools/install)

# Structure
![Frame 1](https://github.com/AxLab-UofC/Laptop-TOIO/assets/66401951/841f6990-a5f2-4c6b-8e87-c6a39817b4cd)

All of the Low-Level Bluetooth Toio is handled within Rust, while the Higher-Level Simulations is done within Processing.

### For AxLab Members
Instructions for registering more toios can be found [here](toioRegistration.md)

### For Users

# Rust Component (Backend)
Our Rust code allows us to connect to toios by their AxLab IDs (Written on the bottom of the toio). We can connect using the following commands when inside the `toio_rust` directory:

## Terminal View
This is what the Laptop-Toio looks like in the terminal:
```
┌ Laptop Toio [Ordered Mode] ─ 157 154 ──────────────────────────── 10.0.0.89:3334 -> 127.0.0.1:3333 ┐
│         ID        Name       Battery       Position         Last Update        Last Command        │
│0        157       41b        100           (201, 130)       >0ms               N/A                 │
│1        154       88e        80            (125, 217)       <50ms              N/A                 │
│                                                                                                    │
│                                                                                                    │
└───────────────────────────────────────────── Quit <Q> ─────────────────────────────────────────────┘
```
- The Top-Left shows what mode Laptop Toio is currently in (See below), as well as the toio connection queue
- The Top-Right shows the ports Laptop Toio is using, in the format `[Host Port] -> [Remote Port]`
- Each Row shows the following info:
    - Index: The index sent to the processing application for updates and commands
    - Toio ID: The Registered AxLab name for the toio (Written on the bottom of the toio)
    - AxLab Name: The internal 3-digit alphanumeric id built into the firmware
    - Battery: Shows the current battery Level of the toio. NOTE: Toios only send battery updates when their motor is not currently running
    - Position: Shows the last registered Position of the toio
    - Last Update and Last Command: Shows the time of the last received update and last sent command

## Unordered Mode
Using the `-a [AxLab IDs]` flag allows us to connect using the AxLab ID:
``` shell
cargo run -- -a 48,46 # Unordered Mode will connect to toios 48 and 46 in any order
```
NOTE: Do not uses spaces between the IDs.

## Ordered Mode
Using the -a [AxLab IDs] -o` flags allows us to connect using the AxLab ID in the order provided. This can be helpful when specific toios should be set for different IDs:
``` shell
cargo run -- -a 48,46 -o # Unordered Mode will connect to toio 48 and then toio 46
```
NOTE: Do not uses spaces between the IDs.


## Search Mode 
- Running Laptop-Toio with the `-s` flag enables 'Search Mode,' which connects to all nearby toios, reducing the hassle of looking over which toios you are.
``` shell
cargo run -- -s # Search Mode will connect will nearby toios
```
NOTE: Don't use this flag if there is anyone else using toios nearby.


## All Flag Options
```
  -a, --axlab-id <AXLAB_ID>  Filter toios by IDs (comma-separated list e.g. 1,2,3)
  -o, --ordered              Use ordered search
  -s, --search               Use unfiltered search
  -p, --port <PORT>          Set receiving port
  -r, --remote <REMOTE>      Set remote port
  -i, --ip <IP>              Set remote IP
  -d, --debug                Show debug terminal stream instead of terminal UI
  -h, --help                 Print help
```

# Processing Component (Application)
The full [toio API](https://toio.github.io/toio-spec/en/docs/about) allows us to send commands and request information from the toio. Most of these API requests have been implemented within Laptop-toio on the Rust and Processing side to allow us to control toios. All of these functions are available on the `Cube` tab of the processing code. All of the toios are stored an array called `cubes`. This means that you can access each toio with `cubes[i]`, where `i` is the order the toios connected in.

## Processing Code Structure
There are 5 tabs in the Laptop-Toio Processing Code:
- The `main` tab includes three core functions: 
    - `settings`: The function allows you to configure your program
    - `setup`: This function is called once at the beginning of your program
    - `draw`: This function is called continously every tick
This tab is analogous to the `setup` and `loop` structure in arduino
- The `cube` tab includes all of the information on the `Cube` struct which stores most of the values and methods used to communicate with toios
- The `events` tab allows you to handle different events from the keyboard, mouse or the toios.
- The `motorVelocityTarget` tab includes the code for the `velocityTarget` function, which is an advanced function used to travel smoothly over time
- The `osc` tab handles the communication with the rust code, and sends commands to the toios.

For the most part, you will purely be writing your code in the `main` and `events` tabs, and `cube` can be refered for you to quickly understand different commands.

## Mat coordinates
In `toio_processing/main.pde`, set `matDims` to `{x1, y1, x2, y2}` using the raw coordinates reported at the physical mat's top-left and bottom-right. The width and height are `x2 - x1` and `y2 - y1`; these values are not screen coordinates or measurements in millimeters. Restart the sketch after changing them.

The display subtracts the mat's origin offset so that its top-left appears at `(45, 45)` in the window. `cubes[i].x`, `cubes[i].y`, and motor targets remain in raw mat coordinates. Mouse clicks are converted back to raw coordinates before sending a target.

## Writing toio Processing code
Processing allows you to write code in two ways:
- The `draw()` function is called continously on a loop. If you want your code to run on a regular interval, this is the best place to put it.
- The `events` tab is comprised of many functions that are called on certain events. 
    - Keyboard and Mouse events call the `keyPressed`, `mousePressed` and `mouseReleased` functions. If you want an extended look at the GUI events that are registered by Processing, you can look at the Keyboard and Mouse sections [here](https://processing.org/reference/#input).
    - toio events automatically call the `buttonDown`, `buttonUp`, `collision` and `doubleTap` functions. These functions will pass the toio IDs as a function parameter, allowing you to use them in your code.

## Toio Updates and Commands
The Rust backend will forward toio updates to the Processing application and forward commands from the Processing application:


### Toio Updates

We automatically recieve updates about:
- [Position](https://toio.github.io/toio-spec/en/docs/ble_id): The values of `cubes[i].x`, `cubes[i].y`, and `cubes[i].theta` will automatically update to the location and angle of each toio.
- [Battery](https://toio.github.io/toio-spec/en/docs/ble_battery): The value of `cubes[i].battery` will automatically update to the battery level of each toio.
- [Button](https://toio.github.io/toio-spec/en/docs/ble_button): The value of `cubes[i].buttonDown` will automatically change on button press of each toio. Changes in this state will also trigger the `buttonDown` and `buttonUp` functions.

We are able to request information from:
- [Motion Sensor](https://toio.github.io/toio-spec/en/docs/ble_sensor)
- [Posture Angle Sensor](https://toio.github.io/toio-spec/en/docs/ble_high_precision_tilt_sensor)
- [Magnetic Sensor](https://toio.github.io/toio-spec/en/docs/ble_magnetic_sensor)

### Toio Commands
We are send commands to control each toio:
- [The Motors](https://toio.github.io/toio-spec/en/docs/ble_motor): NOTE: the maximum speed of a motor on a toio is 115. That commands that can control the toio are:
    -  `cubes[i].motor`: can be called with either `leftSpeed, rightSpeed` or `leftSpeed, rightSpeed, duration`, allowing you to control the speed of each motor individually for a set duration. Further documentation can be found [here](https://toio.github.io/toio-spec/en/docs/ble_motor#motor-control) and [here](https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-specified-duration).
    - `cubes[i].target`: can be called with `x, y, theta`, `mode, x, y, theta` or `control, timeout, mode, maxspeed, speedchange, x, y, theta`, allowing you to target a specific location. Further documentation can be found [here](https://toio.github.io/toio-spec/en/docs/ble_motor#motor-control-with-target-specified).
    - `cubes[i].accelerate`: can be called with `speed, acc, rotateVelocity, rotateDir, dir, priority, duration`, allowing you to control acceleration of the motors. Further documentation can be found [here](https://toio.github.io/toio-spec/en/docs/ble_motor#motor-control-with-acceleration-specified).
    - `cubes[i].multiTarget`: can be called with `mode, targets` or `control, timeout, mode, maxspeed, speedchange, targets` where targets are a 2D array for each target marked as `{x, y}` or `{x, y, theta}`. Unless specified, `theta = 0`. Further documentation can be found [here](https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-multiple-targets-specified).
    - **Advanced:** `cubes[i].velocityTarget` is a function developed by AxLab to allow toiois to follow a continously moving target. When `velocityTarget` is called with `x, y` values every loop, the motion between the targets will be automatically smoothed out. WARNING: This behavior will only occur if the function is called *every* loop. The velocity is caculated by storing values within the `targetxX, `targetY`, `velocityX`, `velocityY` and `velocityTime` variables within the `Cube` class. Read more below in the Velocity Targeting Section
- [The LED Indicator](https://toio.github.io/toio-spec/en/docs/ble_light):
    - `cubes[i].led` can be used to set the LED to a certain color with `duration, red, green, blue` or to produce an LED sequence with `repetitions, lights` where `lights` are a 2D array with each light in the sequence arranged as `{duration, red, green, blue}`. Further documentation can be found [here](https://toio.github.io/toio-spec/en/docs/ble_light).
- [The Speaker](https://toio.github.io/toio-spec/en/docs/ble_sound):
    - `cubes[i].sound` can be called with `soundeffect, volume` to play sound effects. Further documentation can be found [here](https://toio.github.io/toio-spec/en/docs/ble_sound).
    - `cubes[i].midi` can be used to play a certain note with `duration, noteID, volume` or to produce an note sequence with `repetitions, notes` where `note` are a 2D array with each note in the sequence arranged as `{duration, noteID, volume}` or `{duration, noteID}`. Unless specified, `volume = 255`. Further documentation can be found [here](https://toio.github.io/toio-spec/en/docs/ble_sound/#playing-the-midi-note-numbers).

# Advanced Laptop-TOIO Features

## Velocity Targeting
Motor Velocity Target is a special command for toios based off of PID (Proportional Integral Derivative) control to smoothly travel along a path of points. When calling the velocity targeting command, it should be called every frame for a smooth, consistent experience. For example, we can call an `animCircle` function every frame from `draw`:
 
``` java 
int xMid = (int) (matDims[2] + matDims[0]) / 2;
int yMid = (int) (matDims[3] + matDims[1]) / 2;
int radius = 100;
int period = 5; // set an animation period of 5 seconds

void animCircle() {
  int currTime = millis();
  float t = (float) currTime/(5 * 1000);
  float angle = t * (2 * PI);
  
  int x = (int) (xMid + (radius * sin(angle)));
  int y = (int) (yMid + (radius * cos(angle)));
  int theta = (int) ((180 / PI) * (angle + PI/2));
  
  cubes[0].velocityTarget(x, y, theta);
}

void draw() {
    // START TEMPLATE/DEBUG VIEW
    ...
    // END TEMPLATE/DEBUG VIEW

    // INSERT NEW CODE HERE!
    animCircle();
}
```

## Multiple Rust Backends
It is possible to run the Rust Backend on multiple laptops at once and connect it to a single Processing instance. 

1. Find the main host address and client address(es). You can find the current local host on the top right of each instance of Laptop-TOIO:
``` shell
    # Laptop 1
    10.0.0.89:3334 -> 127.0.0.1:3333 # Use the IP address from your main host device

    # Laptop 2
    10.0.0.233:3334 -> 127.0.0.1:3333
```

2. Update your settings to let Processing send messages through your local network. 
    - For Macs, you can enable local network access for Processing under `Settings -> Privacy & Security -> Local Network`

3. On the client device(s), run the Rust backend with the `-i [IP address]` flag to send messages to a remote IP, e.g:
``` shell
   cargo run -- -i 10.0.0.89 ... # Plug in the main host device IP with the `-i` flag
```
After setting the remote IP, the top right should look like this:
``` shell
    # Laptop 2
    10.0.0.233:3334 -> 10.0.0.89:3333
```

4. Update the Processing code to add the additional host(s), as well as `nCubes` and `cubesPerHost`:
``` java
   // main.pde
   int cubesPerHost = 3; // number of toios each instance of the rust backend is connecting to
   int nCubes = 6; // total number of toios connected 

   // osc.pde
   String[] hosts = { "127.0.0.1", "10.0.0.233" }; // list of IP hosts that are connecting to this instance of Laptop-TOIO
```
