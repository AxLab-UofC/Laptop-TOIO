//OSC messages (send)

void aimMotorControl(int cubeId, float x, float y) {
  int hostId = cubeId/cubesPerHost;
  int actualcubeid = cubeId % cubesPerHost;
  OscMessage msg = new OscMessage("/aim");
  msg.add(actualcubeid);
  msg.add((int)x);
  msg.add((int)y);
  oscP5.send(msg, server[hostId]);
}

void basicMotor(int cubeId, boolean leftforwards, int leftspeed, boolean rightforwards, int rightspeed) {
  int hostId = cubeId/cubesPerHost;
  int actualcubeid = cubeId % cubesPerHost;
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

void motorControl(int cubeId, float left, float right, int duration) {
  int hostId = cubeId/cubesPerHost;
  int actualcubeid = cubeId % cubesPerHost;
  OscMessage msg = new OscMessage("/motor");
  msg.add(actualcubeid);
  msg.add((int)left);
  msg.add((int)right);
  msg.add(duration);
  oscP5.send(msg, server[hostId]);
}

void motorTarget(int cubeId, int mode, int x, int y, int theta){
  int hostId = cubeId/cubesPerHost;
  int actualcubeid = cubeId % cubesPerHost;
  OscMessage msg = new OscMessage("/motortarget");
  msg.add(actualcubeid);
  msg.add(mode);
  msg.add(x);
  msg.add(y);
  msg.add(theta);
  oscP5.send(msg, server[hostId]);
}

void motorTarget(int cubeId, int control, int timeout, int mode, int maxspeed, int speedchange,  int x, int y, int theta){
  int hostId = cubeId/cubesPerHost;
  int actualcubeid = cubeId % cubesPerHost;
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

void light(int cubeId, int duration, int red, int green, int blue) {
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

void midi(int cubeId, int noteID, int volume, int duration) {
  int hostId = cubeId/cubesPerHost;
  int actualcubeid = cubeId % cubesPerHost;
  OscMessage msg = new OscMessage("/midi");
  msg.add(actualcubeid);
  msg.add(duration);
  msg.add(noteID);
  msg.add(volume);
  oscP5.send(msg, server[hostId]);
}

void motion(int cubeId) {
  int hostId = cubeId/cubesPerHost;
  int actualcubeid = cubeId % cubesPerHost;
  OscMessage msg = new OscMessage("/motion");
  msg.add(actualcubeid);
  oscP5.send(msg, server[hostId]);
}

void magnetic(int cubeId) {
  int hostId = cubeId/cubesPerHost;
  int actualcubeid = cubeId % cubesPerHost;
  OscMessage msg = new OscMessage("/magnetic");
  msg.add(actualcubeid);
  oscP5.send(msg, server[hostId]);
}

void posture(int cubeId, boolean euler) {
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
