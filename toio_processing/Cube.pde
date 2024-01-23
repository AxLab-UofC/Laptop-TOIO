class Cube {
  int id;
  boolean isActive;
  long lastUpdate;
    
  // position
  int x;
  int y;
  int theta;
  
  //velocity targeting
  int targetx;
  int targety;
  int targetTime;
  
  // battery
  int battery;
  
  // button 
  boolean buttonDown;
  
  // motion
  boolean isHorizontal;
  boolean collision;
  boolean doubleTap;
  int posture;
  int shake;
  
  // magnetic
  int magState;
  int magStrength;
  int magx;
  int magy;
  int magz;
  
  // posture (euler)
  int roll;
  int pitch;
  int yaw;
  
  // posture (quaternions)
  int qx;
  int qy;
  int qw;
  int qz;
  
  Cube(int i) {
    id = i;
    lastUpdate = System.currentTimeMillis();
    isActive = false;
    buttonDown = false;
  }
  
  void checkActive(long now) {
    if (lastUpdate < now - 1500 && isActive) {
      isActive = false;
    }
  }
  
    // Updates position values
  void onPositionUpdate(int upx, int upy, int uptheta) {    
    x = upx;
    y = upy;
    theta = uptheta; 
    
    //insert code here
    
    lastUpdate = System.currentTimeMillis();
    isActive = true;
  }
  
  // Updates battery values
  void onBatteryUpdate(int upbatt) {
    battery = upbatt;
  }
  
  // Updates motion values
  void onMotionUpdate(int flatness, int hit, int double_tap, int face_up, int shake_level) {
     isHorizontal = (flatness == 1);
     collision = (hit == 1);
     doubleTap = (double_tap == 1);
     posture = face_up;
     shake = shake_level;
     
     //insert code here
     
     if (collision) {
       onCollision();
     }
     
     if (doubleTap) {
       onDoubleTap();
     }
  }
  
  // Updates magnetic values 
  void onMagneticUpdate(int upState, int upStrength, int upx, int upy, int upz) {
    magState = upState;
    magStrength = upStrength;
    magx = upx;
    magy = upy;
    magz = upz;
    
    //insert code here
  }
  
  //Updates posture values (euler)
  void onPostureUpdate(int uproll, int uppitch, int upyaw) {
    roll = uproll;
    pitch = uppitch;
    yaw = upyaw;
    
    //insert code here
  }
  
  //Updates posture values (quaternion)
  void onPostureUpdate(int upw, int upx, int upy, int upz) {
    qw = upw;
    qx = upx;
    qy = upy;
    qz = upz;
    
    //insert code here
  }
  
  
  //Execute this code on button press
  void onButtonDown() {
    buttonDown(id);
    buttonDown = true;
    //insert code here
  }
  
  //Execute this code on button release
  void onButtonUp() {
    buttonUp(id);
    buttonDown = false;
    
    //insert code here
  }
  
  //Execute this code on collision
  void onCollision() {
    collision(id);
    
    //insert code here
  }
  
  //Execute this code on double tap
  void onDoubleTap() {
    doubleTap(id);
    
    //insert code here
  }
  
  void onMotorResponse(int control, int response) {
    
    //insert code here
  }
  
  //basic motor control, specification found at:
  //https://toio.github.io/toio-spec/en/docs/ble_motor#motor-control
  //can use negative numbers to move toio backwards
  void motor(int leftSpeed,int rightSpeed) {
    motorBasic(id, leftSpeed, rightSpeed);
  }
  
  //basic motor control w/ duration, specification found at:
  //https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-specified-duration
  //can use negative numbers to move toio backwards
  void motor(int leftSpeed, int rightSpeed, int duration) {
    motorDuration(id, leftSpeed, rightSpeed, duration);
  }
  
  
  //motor control with target specified (simplified), specification found at:
  //https://toio.github.io/toio-spec/en/docs/ble_motor#motor-control-with-target-specified
  //control, timeout, maxspeed, and speed change are preset
  void target(int x, int y, int theta) {
    motorTarget(id, 0, x, y, theta);
  }
  
  //motor control with target specified (simplified), specification found at:
  //https://toio.github.io/toio-spec/en/docs/ble_motor#motor-control-with-target-specified
  //control, timeout, maxspeed, and speed change are preset
  void target(int mode, int x, int y, int theta) {
    motorTarget(id, mode, x, y, theta);
  }
  
  //motor control with target specified (advanced), specification found at:
  //https://toio.github.io/toio-spec/en/docs/ble_motor#motor-control-with-target-specified
  void target(int control, int timeout, int mode, int maxspeed, int speedchange,  int x, int y, int theta) {
    motorTarget(id, control, timeout, mode, maxspeed, speedchange, x, y, theta);
  }
  
  //velocity targeting to allow you to smoothly move to points continously
  boolean velocityTarget(int x, int y) {
    float elapsedTime = millis() - targetTime;
    float vx = (targetx - x) / elapsedTime;
    float vy = (targety - y) / elapsedTime;
    
    boolean val = motorTargetVelocity(id, x, y, vx, vy);
    
    targetx = x;
    targety = y;
    targetTime = millis();
    return val;
  }
  
  //motor control with acceleration specified, specification can be found at:
  //https://toio.github.io/toio-spec/en/docs/ble_motor#motor-control-with-acceleration-specified
  void accelerate(int speed, int acc, int rotateVelocity, int rotateDir, int dir, int priority, int duration) {
    motorAcceleration(id, speed, acc, rotateVelocity, rotateDir, dir, priority, duration);
  }
  
  //motor control with multiple targets specified (simplified), specification found at:
  //https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-multiple-targets-specified
  //targets should be formatted as {x, y, theta} or {x, y}. Unless specified, theta = 0
  void multiTarget(int mode, int[][] targets) {
    motorMultiTarget(id, mode, targets);
  }
  
  //motor control with multiple targets specified (advanced), specification found at:
  //https://toio.github.io/toio-spec/en/docs/ble_motor/#motor-control-with-multiple-targets-specified
  //targets should be formatted as {x, y, theta} or {x, y}. Unless specified, theta = 0
  void multiTarget(int control, int timeout, int mode, int maxspeed, int speedchange,  int[][] targets) {
    motorMultiTarget(id, control, timeout, mode, maxspeed, speedchange, targets);
  }
  
  //Activating the toio LED (single), specification can be found at:
  //https://toio.github.io/toio-spec/en/docs/ble_light
  void led(int duration, int red, int green, int blue) {
    lightLed(id, duration, red, green, blue);
  }
  
  //Activating the toio LED (sequence), specification can be found at:
  //https://toio.github.io/toio-spec/en/docs/ble_light
  //lights should be formatted as {duration, red, green, blue}
  void led(int repetitions, int[][] lights) {
    lightLed(id, repetitions, lights);
  }
  
  //play sound effects, specification can be found at:
  //https://toio.github.io/toio-spec/en/docs/ble_sound
  void sound(int soundeffect, int volume) {
    soundEffect(id, soundeffect, volume);
  }
  
  //play Midi Note (single), specification can be found at:
  //https://toio.github.io/toio-spec/en/docs/ble_sound/#playing-the-midi-note-numbers
  void midi(int duration, int noteID, int volume) {
    soundMidi(id, duration, noteID, volume);
  }
  
  //play Midi Notes (sequence), specification can be found at:
  //https://toio.github.io/toio-spec/en/docs/ble_sound/#playing-the-midi-note-numbers
  //targets should be formatted as {duration, noteID, volume} or {duration, noteID}. Unless specified, volume = 255
  void midi(int repetitions, int[][] notes)  {
    soundMidi(id, repetitions, notes);
  }
  
  float distance(Cube o) {
    return distance(o.x, o.y);
  }

  float distance(float ox, float oy) {
    return sqrt ((x-ox)*(x-ox) + (y-oy)*(y-oy));
  }
}
