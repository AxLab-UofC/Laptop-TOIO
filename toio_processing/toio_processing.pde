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

OscP5 pythonOsc;               // For communication with Python
NetAddress pythonAddress;      // Address for Python

//we'll keep the cubes here
Cube[] cubes;

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

  pythonOsc = new OscP5(this, 4444);   // Different port for Python communication
  pythonAddress = new NetAddress("127.0.0.1", 4445);   // Assuming Python listens here
  pythonOsc.send(new OscMessage("/connected"), pythonAddress);

  
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
  rect(45, 45, 410, 410);

  //draw the cubes
  OscMessage cubePositionsMsg = new OscMessage("/cube_positions");
  for (int i = 0; i < nCubes; i++) {
    cubes[i].checkActive(now);
    if (cubes[i].isActive) {
      pushMatrix();
      translate(cubes[i].x, cubes[i].y);
      
      cubePositionsMsg.add(cubes[i].id);
      cubePositionsMsg.add(cubes[i].x);
      cubePositionsMsg.add(cubes[i].y);
      cubePositionsMsg.add(cubes[i].theta);
        
      rotate(cubes[i].theta * PI/180);
      rect(-10, -10, 20, 20);
      line(0, 0, 20, 0);
      popMatrix();
    }
  }
  //TODO: send positions 
  pythonOsc.send(cubePositionsMsg, pythonAddress);
  //END DO NOT EDIT
  //start your code
  //motorBasic(1, 100, 100);
}
