import oscP5.*;
import netP5.*;




//// Instruction for Windows Users  (Feb 2. 2025) ////
// 1. Enable WindowsMode and set nCubes to the exact number of toio you are connecting.
// 2. Run Processing Code FIRST, Then Run the Rust Code. After running the Rust Code, you should place the toio on the toio mat, then Processing should start showing the toio position.
// 3. When you re-run the processing code, make sure to stop the rust code and toios to be disconnected (switch to Bluetooth stand-by mode [blue LED blinking]). If toios are taking time to disconnect, you can optionally turn off the toio and turn back on using the power button.
// Optional: If the toio behavior is werid consider dropping the framerate (e.g. change from 30 to 10)
// 
boolean WindowsMode = false; //When you enable this, it will check for connection with toio via Rust first, before starting void loop()

int framerate = 30;

int[] matDims = { 45, 45, 455, 455 }; // Mat Dimensions, expressed as { x1, y1, x2, y2 };
String[] hosts = { "127.0.0.1" }; // list of IP hosts that are connecting to this instance of Laptop-TOIO


OscP5 oscP5; // for OSC
NetAddress[] server; // where to send the commands to

// constants
// The soft limit on how many toios a laptop can handle is in the 10-12 range
// the more toios you connect to, the more difficult it becomes to sustain the connection
int nCubes = 4;
int cubesPerHost = 12;
int maxMotorSpeed = 115;
int xOffset = matDims[0] - 45;
int yOffset = matDims[1] - 45;

Cube[] cubes;

void settings() {
  size(matDims[2] + 45, matDims[3] + 45);
}


void setup() {
  //create OSC servers
  oscP5 = new OscP5(this, 3333);
  server = new NetAddress[hosts.length];
  for (int i = 0; i < hosts.length; i++) {
    server[i] = new NetAddress(hosts[i], 3334);
  }
  
  //create cubes
  cubes = new Cube[nCubes];
  for (int i = 0; i< nCubes; ++i) {
    cubes[i] = new Cube(i);
  }

  //do not send TOO MANY PACKETS
  //we'll be updating the cubes every frame, so don't try to go too high
  frameRate(framerate);
  if (WindowsMode) check_connection();
}

void draw() {
  // START TEMPLATE/DEBUG VIEW
  background(255);
  stroke(0);
  long now = System.currentTimeMillis();

  // draw the "mat"
  fill(255);
  rect(matDims[0] - xOffset, matDims[1] - yOffset, matDims[2] - matDims[0], matDims[3] - matDims[1]);

  // draw the cubes
  pushMatrix();
  translate(xOffset, yOffset);
  
  for (int i = 0; i < nCubes; i++) {
    cubes[i].checkActive(now);
    
    if (cubes[i].isActive) {
      pushMatrix();
      translate(cubes[i].x, cubes[i].y);
      
      // draw toio label
      fill(0);
      textSize(15);
      text(i, 0, -20);
      
      // draw toio cube
      noFill();
      rotate(cubes[i].theta * PI/180);
      rect(-10, -10, 20, 20);
      line(0, 0, 20, 0);
      popMatrix();
    }
  }
  popMatrix();
  // END TEMPLATE/DEBUG VIEW
  
  // INSERT NEW CODE HERE!
}
