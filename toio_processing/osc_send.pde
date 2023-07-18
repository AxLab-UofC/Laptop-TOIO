//OSC messages (send)

//this function has been (mostly) depreciated)
//void aimMotorControl(int cubeId, float x, float y) {
//  int hostId = cubeId/cubesPerHost;
//  int actualcubeid = cubeId % cubesPerHost;
//  OscMessage msg = new OscMessage("/aim");
//  msg.add(actualcubeid);
//  msg.add((int)x);
//  msg.add((int)y);
//  oscP5.send(msg, server[hostId]);
//}


//basic motor control (simplified), specification found at:
//https://toio.github.io/toio-spec/en/docs/ble_motor#motor-control
//this controls the speed of the left motor, the the speed of the right motor, and the duration of the movement
//setting value to negative will move motor backwards
void motorControl(int cubeId, float leftspeed, float rightspeed, int duration) {
  int hostId = cubeId/cubesPerHost;
  int actualcubeid = cubeId % cubesPerHost;
  
   if (!cubes[actualcubeid].onFloor) {
    float tempspeed = rightspeed;
    rightspeed = leftspeed;
    leftspeed = tempspeed;
  }
  
  OscMessage msg = new OscMessage("/motor");
  msg.add(actualcubeid);
  msg.add((int)leftspeed);
  msg.add((int)rightspeed);
  msg.add(duration);
  oscP5.send(msg, server[hostId]);
}

//basic motor control (advanced), specification found at:
//https://toio.github.io/toio-spec/en/docs/ble_motor#motor-control
void basicMotor(int cubeId, boolean leftforwards, int leftspeed, boolean rightforwards, int rightspeed) {
  int hostId = cubeId/cubesPerHost;
  int actualcubeid = cubeId % cubesPerHost;
  
  if (!cubes[actualcubeid].onFloor) {
    int tempspeed = rightspeed;
    rightspeed = leftspeed;
    leftspeed = tempspeed;
    
    boolean tempforwards = rightforwards;
    rightforwards = leftforwards;
    leftforwards = tempforwards;
  }
  
  OscMessage msg = new OscMessage("/motorbasic");
  msg.add(actualcubeid);
  if (leftforwards) {
    msg.add(0x01);
  } else {
    msg.add(0x02);
  }
  msg.add(leftspeed);
    if (rightforwards) {
    msg.add(0x01);
  } else {
    msg.add(0x02);
  }
  msg.add(rightspeed);
  oscP5.send(msg, server[hostId]);
}

//motor control with target specified (simplified), specification found at:
//https://toio.github.io/toio-spec/en/docs/ble_motor#motor-control-with-target-specified
//control, timeout, maxspeed, and speed change are preset
void motorTarget(int cubeId, int mode, int x, int y, int theta){
  int hostId = cubeId/cubesPerHost;
  int actualcubeid = cubeId % cubesPerHost;
  
   if (!cubes[actualcubeid].onFloor) {
    y = ymax - y;
    theta = 360 - theta;
  }
  
  OscMessage msg = new OscMessage("/motortarget");
  msg.add(actualcubeid);
  msg.add(mode);
  msg.add(x);
  msg.add(y);
  msg.add(theta);
  oscP5.send(msg, server[hostId]);
}

//motor control with target advanced (simplified), specification found at:
//https://toio.github.io/toio-spec/en/docs/ble_motor#motor-control-with-target-specified
void motorTarget(int cubeId, int control, int timeout, int mode, int maxspeed, int speedchange,  int x, int y, int theta){
  int hostId = cubeId/cubesPerHost;
  int actualcubeid = cubeId % cubesPerHost;
  
  if (!cubes[actualcubeid].onFloor) {
    y = ymax - y;
    theta = 360 - theta;
  }
  
  OscMessage msg = new OscMessage("/motortarget");
  msg.add(actualcubeid);
  msg.add(control);
  msg.add(timeout);
  msg.add(mode);
  msg.add(maxspeed);
  msg.add(speedchange);
  msg.add(x);
  msg.add(y);
  msg.add(theta);
  oscP5.send(msg, server[hostId]);
}

//would need to change rotational velocity for threading space project, can be done if we need this function
//motor control with acceleration specified, specification can be found at:
//https://toio.github.io/toio-spec/en/docs/ble_motor#motor-control-with-acceleration-specified
void motorAcceleration(int cubeId, int speed, int a, int rotateVelocity, int rotateDir, int dir, int priority, int duration){
  OscMessage msg = new OscMessage("/motoracceleration");
  int hostId = cubeId/cubesPerHost;
  int actualcubeid = cubeId % cubesPerHost;
  msg.add(actualcubeid);
  msg.add(speed);
  msg.add(a);
  msg.add(rotateVelocity);
  msg.add(rotateDir);
  msg.add(dir);
  msg.add(priority);
  msg.add(duration);
  oscP5.send(msg, server[hostId]);
  oscP5.send(msg, server[hostId]);
}

//activating LED on bottom of toio, specification can be found at:
//https://toio.github.io/toio-spec/en/docs/ble_light
void led(int cubeId, int duration, int red, int green, int blue) {
  int hostId = cubeId/cubesPerHost;
  int actualcubeid = cubeId % cubesPerHost;
  OscMessage msg = new OscMessage("/led");
  msg.add(actualcubeid);
  msg.add(duration);
  msg.add(red);
  msg.add(green);
  msg.add(blue);
  oscP5.send(msg, server[hostId]);
}


//id of different sound effects can be found at:
//https://toio.github.io/toio-spec/en/docs/ble_sound
void sound(int cubeId, int soundeffect, int volume) {
  int hostId = cubeId/cubesPerHost;
  int actualcubeid = cubeId % cubesPerHost;
  OscMessage msg = new OscMessage("/sound");
  msg.add(actualcubeid);
  msg.add(soundeffect);
  msg.add(volume);
  oscP5.send(msg, server[hostId]);
}

//id for different midi notes can be found at:
//https://toio.github.io/toio-spec/en/docs/ble_sound/#playing-the-midi-note-numbers
void midi(int cubeId, int duration, int noteID, int volume) {
  int hostId = cubeId/cubesPerHost;
  int actualcubeid = cubeId % cubesPerHost;
  OscMessage msg = new OscMessage("/midi");
  msg.add(actualcubeid);
  msg.add(duration);
  msg.add(noteID);
  msg.add(volume);
  oscP5.send(msg, server[hostId]);
}

//request for motion detection information, specification can be found at:
//https://toio.github.io/toio-spec/en/docs/ble_sensor
void motionRequest(int cubeId) {
  int hostId = cubeId/cubesPerHost;
  int actualcubeid = cubeId % cubesPerHost;
  OscMessage msg = new OscMessage("/motion");
  msg.add(actualcubeid);
  oscP5.send(msg, server[hostId]);
}

//request for magnetic sensor information, specification can be found at:
//https://toio.github.io/toio-spec/en/docs/ble_magnetic_sensor
void magneticRequest(int cubeId) {
  int hostId = cubeId/cubesPerHost;
  int actualcubeid = cubeId % cubesPerHost;
  OscMessage msg = new OscMessage("/magnetic");
  msg.add(actualcubeid);
  oscP5.send(msg, server[hostId]);
}

//request for cube rotation information, specification can be found at:
//https://toio.github.io/toio-spec/en/docs/ble_high_precision_tilt_sensor
void postureRequest(int cubeId, boolean euler) {
    int hostId = cubeId/cubesPerHost;
    int actualcubeid = cubeId % cubesPerHost;
    OscMessage msg;
    if (euler) {
      msg = new OscMessage("/postureeuler");
    } else {
      msg = new OscMessage("/posturequaternion");
    }
     msg.add(actualcubeid);
     oscP5.send(msg, server[hostId]);
}
