enum moveStatus {
  NONE, INPROGRESS, COMPLETE, ERROR
}

class Cube {
  int id;
  boolean isActive;
  long lastUpdate;
  
  //targeting
  moveStatus status = moveStatus.NONE;
  int control = -1;
    
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
  
  boolean onFloor = true;
  
  Cube(int i) {
    id = i;
    lastUpdate = System.currentTimeMillis();
    isActive = false;
  }
  
  void checkActive(long now) {
    if (lastUpdate < now - 1500 && isActive) {
      isActive = false;
      led(id, 0, 255, 255, 255);
    }
  }
  
  void motor(boolean leftforwards, int leftspeed, boolean rightforwards, int rightspeed) {
    motorBasic(id, leftforwards, leftspeed, rightforwards, rightspeed);
  }
  
  void duration(int speed, int duration) {
    motorDuration(id, speed, duration);
  }
  
  void acceleration(int speed, int a, int rotateVelocity, int rotateDir, int dir, int priority, int duration) {
    motorAcceleration(id, speed, a, rotateVelocity, rotateDir, dir, priority, duration);
  }
  
  void target(int mode, int x, int y, int theta) {
    status = moveStatus.INPROGRESS;
    motorTarget(id, mode, x, y, theta);
  }
  
  void target(int control, int timeout, int mode, int maxspeed, int speedchange,  int x, int y, int theta) {
    status = moveStatus.INPROGRESS;
    motorTarget(id, control, timeout, mode, maxspeed, speedchange, x, y, theta);
  }
  
  void multiTarget(int mode, int[][] targets) {
    status = moveStatus.INPROGRESS;
    motorMultiTarget(id, mode, targets);
  }
  
  void multiTarget(int control, int timeout, int mode, int maxspeed, int speedchange,  int[][] targets) {
    status = moveStatus.INPROGRESS;
    motorMultiTarget(id, control, timeout, mode, maxspeed, speedchange, targets);
  }
  
    // Updates position values
  void positionUpdate(int upx, int upy, int uptheta) {    
    x = upx;
    y = upy;
    theta = uptheta; 
    lastUpdate = System.currentTimeMillis();
    
    if(!isActive) {
      isActive = true;
      if (onFloor) {
        led(id, 0, 255, 0, 0);
      } else {
        led(id, 0, 0, 0, 255);
      }
    }
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
  
  void motorUpdate(int upcontrol, int upresponse) {
    control = upcontrol;
    switch(upresponse) {
      case 0:
        status = moveStatus.COMPLETE;
        break;
        
      case 5:
        status = moveStatus.NONE;
        break;
        
      default:
        status = moveStatus.ERROR;
        break;
    }
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
    //println("Collision Detected!");
  }
  
  //Execute this code on double tap
  void onDoubleTap() {
    //println("Double Tap Detected!");
  }
  
}
