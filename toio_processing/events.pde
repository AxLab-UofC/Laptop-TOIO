//execute code on key pressed
void keyPressed() {

  switch(key) {
  case '1': //raw motor control

    //basic motor control w/ duration, specification found at:
    //https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-specified-duration
    //can use negative numbers to move toio backwards
    // void motor(int leftSpeed, int rightSpeed, int duration)

    cubes[0].motor(115, 115, 5);
    break;

  case '2': //targeting control


    // void target(int x, int y, int theta)

    //motor control with target specified (simplified), specification found at:
    //https://toio.github.io/toio-spec/en/docs/ble_motor#motor-control-with-target-specified
    //control, timeout, maxspeed, and speed change are preset
    cubes[0].target(200, 200, 270);

    break;

  case '3': //multi-targeting control

    //motor control with multiple targets specified (simplified), specification found at:
    //https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-multiple-targets-specified
    //targets should be formatted as {x, y, theta} or {x, y}. Unless specified, theta = 0
    // void multiTarget(int mode, int[][] targets)


    int[][] targets = {{200, 200}, {200, 300, 90}, {300, 300}, {300, 200, 270}, {200, 200, 180}};
    cubes[0].multiTarget(0, targets);
    break;

  case '4': // LED control

    //Activating the toio LED (single), specification can be found at:
    //https://toio.github.io/toio-spec/en/docs/ble_light
    // void led(int duration, int red, int green, int blue)

    cubes[0].led(100, 255, 255, 255);
    break;

  case '5': // Sequence-LED Control

    //Activating the toio LED (sequence), specification can be found at:
    //https://toio.github.io/toio-spec/en/docs/ble_light
    //lights should be formatted as {duration, red, green, blue}
    // void led(int repetitions, int[][] lights)

    int[][] lights = {{30, 0, 255, 0}, {30, 0, 0, 255}};
    cubes[0].led(5, lights);
    break;

  case '6': // Sound Effect

    //play sound effects, specification can be found at:
    //https://toio.github.io/toio-spec/en/docs/ble_sound
    // sound(int soundeffect, int volume) {

    cubes[0].sound(2, 255);
    break;

  case '7': // Single Midi Tone

    //play Midi Note (single), specification can be found at:
    //https://toio.github.io/toio-spec/en/docs/ble_sound/#playing-the-midi-note-numbers
    // void midi(int duration, int noteID, int volume)
    cubes[0].midi(10, 69, 255);
    break;

  case '8': // Sequencial Midi Tones

    //play Midi Notes (sequence), specification can be found at:
    //https://toio.github.io/toio-spec/en/docs/ble_sound/#playing-the-midi-note-numbers
    //targets should be formatted as {duration, noteID, volume} or {duration, noteID}. Unless specified, volume = 255
    // void midi(int repetitions, int[][] notes)
    int[][] notes = {{30, 64, 20}, {30, 63, 20}, {30, 64, 20}, {30, 63, 20}, {30, 64, 20}, {30, 63, 20}, {30, 59, 20}, {30, 62, 20}, {30, 60, 20}, {30, 57, 20}};
    cubes[0].midi(1, notes);
    break;

  default:
    break;
  }
}

//execute code when mouse is pressed
void mousePressed() {
  if (mouseX > 45 && mouseX < matDimension[2] - xOffset && mouseY > 45 && mouseY < matDimension[2] - yOffset) {
    cubes[0].target(mouseX, mouseY, 0);
  }

  //insert code here;
}

//execute code when mouse is released
void mouseReleased() {
  //insert code here;
}

//execute code when button on toio is pressed
void buttonDown(int id) {
  println("Button Pressed!");
}

//execute code when button on toio is released
void buttonUp(int id) {
  println("Button Released!");

}

//execute code when toio detects collision
void collision(int id) {
  println("Collision Detected!");

}

//execute code when toio detects double tap
void doubleTap(int id) {
  println("Double Tap Detected!");

}
