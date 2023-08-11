import oscP5.*;
import netP5.*;


//constants
//The soft limit on how many toios a laptop can handle is in the 10-12 range
//the more toios you connect to, the more difficult it becomes to sustain the connection
int nCubes = 12;
int cubesPerHost = 12;

int xmax = 949;
int ymax = 898;


//for OSC
OscP5 oscP5;
//where to send the commands to
NetAddress[] server;

//we'll keep the cubes here
Cube[] cubes;
Pair[] pairs;

void settings() {
  size(1000, 1000, P3D);
}


void setup() {
  //launch OSC sercer
  oscP5 = new OscP5(this, 3333);
  server = new NetAddress[1];
  server[0] = new NetAddress("127.0.0.1", 3334);

  //create cubes
  cubes = new Cube[nCubes];
  for (int i = 0; i< cubes.length; ++i) {
    cubes[i] = new Cube(i);
  }
  
  pairs = new Pair[6];
  for (int i = 0; i < 6; i++) {
     pairs[i] = new Pair((i * 2), (i * 2) + 1);
     println((i * 2), (i * 2) + 1);
  }


  //do not send TOO MANY PACKETS
  //we'll be updating the cubes every frame, so don't try to go too high
  frameRate(30);
}

void draw() {
  //START DO NOT EDIT
  background(255);
  stroke(0);
  long now = System.currentTimeMillis();

  //draw the "mat"
  fill(255);
  rect(45, 45, xmax, ymax);

  //draw the cubes
  for (int i = 0; i < nCubes; i++) {
    cubes[i].checkActive(now);
    
    if (cubes[i].isActive) {
      pushMatrix();
      if (cubes[i].onFloor) {
        stroke(255, 0, 0);
        translate(cubes[i].x, cubes[i].y);
      } 
      else {
        stroke(0, 0, 255);
        translate(cubes[i].x, ymax - cubes[i].y);
      }
      
      
      rotate(cubes[i].theta * PI/180);
      rect(-10, -10, 20, 20);
      line(0, 0, 20, 0);
      popMatrix();
    }
  }
  //END DO NOT EDIT
}
