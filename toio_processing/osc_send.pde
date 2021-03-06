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
