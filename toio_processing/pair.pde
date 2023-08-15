class Pair {
  Cube t;
  Cube b;
  
  Pair(int top, int bottom) {
    t = cubes[top];
    cubes[top].onFloor = false;
    b = cubes[bottom];
  }
  
  void checkActive(long now) {
    t.checkActive(now);
    b.checkActive(now);
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
  
  void multiTarget(int mode, int[][] targets) {
    t.multiTarget(mode, targets);
    b.multiTarget(mode, targets);
  }
  
  void multiTarget(int control, int timeout, int mode, int maxspeed, int speedchange,  int[][] targets) {
    t.multiTarget(control, timeout, mode, maxspeed, speedchange, targets);
    b.multiTarget(control, timeout, mode, maxspeed, speedchange, targets);
  }
  
  void acceleration(int speed, int a, int rotateVelocity, int rotateDir, int dir, int priority, int duration) {
    t.acceleration(speed, a, rotateVelocity, rotateDir, dir, priority, duration);
    b.acceleration(speed, a, rotateVelocity, rotateDir, dir, priority, duration);
  }
}

void moveTargets(int[][] spots) {
   for (int i = 0; i < spots.length; i++) {
    motorTarget(i, 0, spots[i][0], spots[i][1], spots[i][2]);
  }
}

void moveTargets(float[][] spots) {
   for (int i = 0; i < spots.length; i++) {
    motorTarget(i, 0, int(spots[i][0]), int(spots[i][1]), int(spots[i][2]));
    println(i, int(spots[i][0]), int(spots[i][1]), int(spots[i][2]));
  }
}

void multiMovePairs(int[][] spots) {
   for(int i = 0;i < spots.length; i++) {
    pairs[i].multiTarget(0, spots);
  }
}

void movePairs(int[][] spots) {
  for(int i = 0;i < spots.length; i++) {
    pairs[i].target(0, spots[i][0], spots[i][1], spots[i][2]);
  }
}

void movePairs(float[][] spots) {
  for(int i = 0;i < spots.length; i++) {
    pairs[i].target(0, int(spots[i][0]), int(spots[i][1]), int(spots[i][2]));
  }
}

void moveTop(int[][] spots) {
  for(int i = 0;i < spots.length; i++) {
    pairs[i].t.target(0, spots[i][0], spots[i][1], spots[i][2]);
  }
}

void moveTop(float[][] spots) {
  for(int i = 0;i < spots.length; i++) {
    pairs[i].t.target(0, int(spots[i][0]), int(spots[i][1]), int(spots[i][2]));
  }
}

void moveBottom(int[][] spots) {
  for(int i = 0;i < spots.length; i++) {
    pairs[i].b.target(0, spots[i][0], spots[i][1], spots[i][2]);
  }
}

void moveBottom(float[][] spots) {
  for(int i = 0;i < spots.length; i++) {
    pairs[i].b.target(0, int(spots[i][0]), int(spots[i][1]), int(spots[i][2]));
  }
}

void stop() {
   for (int i = 0; i < cubes.length; i++) {
    motorBasic(i, 0, 0);
  }
}
