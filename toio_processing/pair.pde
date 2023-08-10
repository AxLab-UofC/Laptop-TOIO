class Half {
  int id;
  
  Half(int i) {
    id = i;
  }
  
  void motor(boolean leftforwards, int leftspeed, boolean rightforwards, int rightspeed) {
    motorBasic(id, leftforwards, leftspeed, rightforwards, rightspeed);
  }
  
  void target(int mode, int x, int y, int theta) {
    motorTarget(id, mode, x, y, theta);
  }
  
  void target(int control, int timeout, int mode, int maxspeed, int speedchange,  int x, int y, int theta) {
    motorTarget(id, control, timeout, mode, maxspeed, speedchange, x, y, theta);
  }
  
  void acceleration(int speed, int a, int rotateVelocity, int rotateDir, int dir, int priority, int duration) {
    motorAcceleration(id, speed, a, rotateVelocity, rotateDir, dir, priority, duration);
  }
}


class Pair {
  Half t;
  Half b;
  
  Pair(int top, int bottom) {
    t = new Half(top);
    cubes[top].onFloor = false;
    b = new Half(bottom);
  }
  
    void motor(boolean leftforwards, int leftspeed, boolean rightforwards, int rightspeed) {
    t.motor(leftforwards, leftspeed, rightforwards, rightspeed);
    b.motor(leftforwards, leftspeed, rightforwards, rightspeed);
  }
  
  void target(int mode, int x, int y, int theta) {
    t.target(mode, x, y, theta);
    b.target(mode, x, y, theta);
  }
  
  void target(int control, int timeout, int mode, int maxspeed, int speedchange,  int x, int y, int theta) {
    t.target(control, timeout, mode, maxspeed, speedchange, x, y, theta);
    b.target(control, timeout, mode, maxspeed, speedchange, x, y, theta);
  }
  
  void acceleration(int speed, int a, int rotateVelocity, int rotateDir, int dir, int priority, int duration) {
    t.acceleration(speed, a, rotateVelocity, rotateDir, dir, priority, duration);
    b.acceleration(speed, a, rotateVelocity, rotateDir, dir, priority, duration);
  }
  
}
