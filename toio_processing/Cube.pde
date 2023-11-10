class Cube {
  int id;
  boolean isActive;
  long lastUpdate;
    
  // position
  int x;
  int y;
  int theta;
  
  // battery
  int battery;
  
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
    println("Button Pressed!");
    
    //insert code here
  }
  
  //Execute this code on button release
  void onButtonUp() {
    println("Button Released");
    
    //insert code here
  }
  
  //Execute this code on collision
  void onCollision() {
    println("Collision Detected!");
    
    //insert code here
  }
  
  //Execute this code on double tap
  void onDoubleTap() {
    println("Double Tap Detected!");
    
    //insert code here
  }
  
  void motor(int leftSpeed,int rightSpeed) {
    motorBasic(id, leftSpeed, rightSpeed);
  }
  
  void motor(int leftSpeed, int rightSpeed, int duration) {
    motorDuration(id, leftSpeed, rightSpeed, duration);
  }
  
  void target(int mode, int x, int y, int theta) {
    motorTarget(id, mode, x, y, theta);
  }
  
  void target(int control, int timeout, int mode, int maxspeed, int speedchange,  int x, int y, int theta) {
    motorTarget(id, control, timeout, mode, maxspeed, speedchange, x, y, theta);
  }
  
  void accelerate(int speed, int a, int rotateVelocity, int rotateDir, int dir, int priority, int duration) {
    motorAcceleration(id, speed, a, rotateVelocity, rotateDir, dir, priority, duration);
  }
  
  void multiTarget(int mode, int[][] targets) {
    motorMultiTarget(id, mode, targets);
  }
  
  void multiTarget(int control, int timeout, int mode, int maxspeed, int speedchange,  int[][] targets) {
    motorMultiTarget(id, control, timeout, mode, maxspeed, speedchange, targets);
  }
  
  void led(int duration, int red, int green, int blue) {
    lightLed(id, duration, red, green, blue);
  }
  
  void led(int repetitions, int[][] lights) {
    lightLed(id, repetitions, lights);
  }
  
  void sound(int soundeffect, int volume) {
    soundEffect(id, soundeffect, volume);
  }
  
  void midi(int duration, int noteID, int volume) {
    soundMidi(id, duration, noteID, volume);
  }
  
  void midi(int repetitions, int[][] notes)  {
    soundMidi(id, repetitions, notes);
  }
}
