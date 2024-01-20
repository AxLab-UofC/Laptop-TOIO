import oscP5.*;
import netP5.*;


//constants
//The soft limit on how many toios a laptop can handle is in the 10-12 range
//the more toios you connect to, the more difficult it becomes to sustain the connection
int nCubes = 12;
int cubesPerHost = 12;


//for OSC
OscP5 oscP5;
//where to send the commands to
NetAddress[] server;

//we'll keep the cubes here
Cube[] cubes;

void settings() {
  size(1000, 1000);
}


void setup() {
  //launch OSC sercer
  oscP5 = new OscP5(this, 3333);
  server = new NetAddress[1];
  server[0] = new NetAddress("127.0.0.1", 3334);

  //create cubes
  cubes = new Cube[nCubes];
  for (int i = 0; i< nCubes; ++i) {
    cubes[i] = new Cube(i);
  }

  //do not send TOO MANY PACKETS
  //we'll be updating the cubes every frame, so don't try to go too high
  frameRate(30);
}

void draw() {
  //START TEMPLATE/DEBUG VIEW
  background(255);
  stroke(0);
  long now = System.currentTimeMillis();
  postureRequest(0);

  //draw the "mat"
  fill(255);
  rect(45, 45, 410, 410);

  //draw the cubes
  for (int i = 0; i < nCubes; i++) {
    cubes[i].checkActive(now);
    
    if (cubes[i].isActive) {
      pushMatrix();
      translate(cubes[i].x, cubes[i].y);
      fill(0);
      textSize(15);
      text(i, 0, -20);
      noFill();
      rotate(cubes[i].theta * PI/180);
      rect(-10, -10, 20, 20);
      line(0, 0, 20, 0);
      popMatrix();
    }
  }
  //END TEMPLATE/DEBUG VIEW
  
  //insert code here
  if (cubes[0].posture == 1) {
    cubes[0].led(0, 255, 0, 0);
  }
}
