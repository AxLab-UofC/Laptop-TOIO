import peasy.PeasyCam;
PeasyCam cam;


import oscP5.*;
import netP5.*;

boolean zorozoro = true;
int[][] zoropairs = {{185,137},{105,171},{118,92},{190,145},{127,144},{172,148}};

//constants
//The soft limit on how many toios a laptop can handle is in the 10-12 range
//the more toios you connect to, the more difficult it becomes to sustain the connection
int nCubes = 200;
int nPairs = 6;
int cubesPerHost = nCubes;



int xmax = 949;
int ymax = 898;
int vert = 500;

color StringCol = (0);
color backgroundCol = (180);
color ceilingCol = color(225, 225, 255);
color floorCol = color(255, 225, 225);

int stringWeight = 3;


//for OSC
OscP5 oscP5;
//where to send the commands to
NetAddress[] server;

//we'll keep the cubes here
Cube[] cubes;
Pair[] pairs;

//For new Mac silicon chip to render 3D correctly:
import com.jogamp.opengl.GLProfile;
{
  GLProfile.initSingleton();
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
  
  pairs = new Pair[nPairs];
  if (zorozoro) {
     for (int i = 0; i < zoropairs.length; i++) {
     pairs[i] = new Pair(zoropairs[i][0], zoropairs[i][1]); // For Zorozoro
     println((i * 2), (i * 2) + 1);
    }
  } else {
      for (int i = 0; i < nPairs; i++) {
       pairs[i] = new Pair((i * 2), (i * 2) + 1); //For Laptop-TOIO
       println((i * 2), (i * 2) + 1);
    }
  }



  //do not send TOO MANY PACKETS
  //we'll be updating the cubes every frame, so don't try to go too high
  fullScreen(P3D);
  cam = new PeasyCam(this, 400);
  cam.setDistance(2000);
  cam.rotateX(-PI/2);

  smooth();
  
  frameRate(30);
}

void draw() {
  if (keyPressed && key == ' ') {
    cam.setActive(true);
  } else {
    cam.setActive(false);
  }
  
  //START DO NOT EDIT
  background (backgroundCol);
  noStroke();
  fill(200);
  rectMode(CENTER);
  background(255);
  
  
  long now = System.currentTimeMillis();

  //draw the "mat"
  background (backgroundCol);
  noStroke();
  fill(200);
  rectMode(CENTER);

  pushMatrix();
  fill(floorCol);
  translate(0, 0, -vert);
  rect(0, 0, xmax, ymax);
  popMatrix();
  
  pushMatrix();
  fill(ceilingCol);
  translate(0, 0, vert);
  rect(0, 0, xmax, ymax);
  popMatrix();
 
  
  
  for (int i = 0; i < nPairs; i++) {
    pairs[i].checkActive(now);
    boolean topActive = pairs[i].t.isActive;
    boolean bottomActive = pairs[i].b.isActive;
    
    pushMatrix();
    translate(-xmax/2,-ymax/2, 0); 
  stroke(200);
  fill(255);
  strokeWeight(1);
  
    //Draw Top Toio
    if (topActive) {
      pushMatrix();
        translate(pairs[i].t.x, ymax - pairs[i].t.y, vert - 4);
        rotate(pairs[i].t.theta * PI/180);
        stroke(200);
        fill(255);
        strokeWeight(1);
        box(12, 12, 7);
      popMatrix();
    }

    
    //Draw Bottom Toio
    if (bottomActive) {
      pushMatrix();
        translate(pairs[i].b.x, pairs[i].b.y, -vert + 4);
        rotate(pairs[i].b.theta * PI/180);
        stroke(200);
        fill(255);
        strokeWeight(1);
        box(12, 12, 7);
      popMatrix();
    }

    
    //Draw strings
    if (topActive && bottomActive) {
      stroke(StringCol);
      strokeWeight(stringWeight);
      line(pairs[i].t.x, ymax - pairs[i].t.y, vert, pairs[i].b.x, pairs[i].b.y, -vert);
    }
    
    popMatrix();
  }
  
  cam.beginHUD();
  cam.endHUD();
  //END DO NOT EDIT
}
