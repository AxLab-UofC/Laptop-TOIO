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
  void positionUpdate(int upx, int upy, int uptheta) {    
    x = upx;
    y = upy;
    theta = uptheta; 
    
    lastUpdate = System.currentTimeMillis();
    isActive = true;
  }
  
  // Updates battery values
  void batteryUpdate(int upbatt) {
    battery = upbatt;
  }
  
  // Updates motion values
  void motionUpdate(int flatness, int hit, int double_tap, int face_up, int shake_level) {
     isHorizontal = (flatness == 1);
     collision = (hit == 1);
     doubleTap = (double_tap == 1);
     posture = face_up;
     shake = shake_level;
     
     if (collision) {
       onCollision();
     }
     
     if (doubleTap) {
       onDoubleTap();
     }
  }
  
  // Updates magnetic values 
  void magneticUpdate(int upState, int upStrength, int upx, int upy, int upz) {
    magState = upState;
    magStrength = upStrength;
    magx = upx;
    magy = upy;
    magz = upz;
  }
  
  //Updates posture values (euler)
  void postureUpdate(int uproll, int uppitch, int upyaw) {
    roll = uproll;
    pitch = uppitch;
    yaw = upyaw;
  }
  
  //Updates posture values (quaternion)
  void postureUpdate(int upw, int upx, int upy, int upz) {
    qw = upw;
    qx = upx;
    qy = upy;
    qz = upz;
  }
  
  
  //Execute this code on button press
  void buttonDown() {
    println("Button Pressed!");
    
  }
  
  //Execute this code on button release
  void buttonUp() {
    println("Button Released");
  }
  
  //Execute this code on collision
  void onCollision() {
    println("Collision Detected!");
  }
  
  //Execute this code on double tap
  void onDoubleTap() {
    println("Double Tap Detected!");
  }
  
}
